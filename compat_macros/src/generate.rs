extern crate proc_macro;

use proc_macro2::Span;
use quote::{format_ident, quote, quote_spanned};
use syn::{
    Attribute, DataEnum, DataStruct, DeriveInput, Error, Expr, Fields, Ident, Lit, Meta, Path,
    Result, Token, Type,
    punctuated::Punctuated,
    spanned::Spanned as _
};

use crate::{
    placeholder::replace_compat_placeholder,
    symbols::{COMPAT_ATTR, COMPAT_PREFIX, PLACEHOLDER, MAP, NAME, TYPE},
};

const NAMED_FIELDS_ONLY: &'static str = "GenerateCompat only supports structs with named fields.";

/// Generates a semver-6.0.0 compatible struct for serialization purposes.
pub(crate) fn generate_compat_struct(ast: &DeriveInput, data: &DataStruct)
    -> proc_macro2::TokenStream
{
    let parsed_outer = match parse_outer_compat_attr(&ast.attrs) {
        Ok(parsed) => parsed,
        Err(err) => return err.into_compile_error(),
    };

    // Stores the compat struct's field names where the index into the Vec corresponds to the
    // original (legacy) struct's field's name.
    //
    // These will only differ when a field is flagged with #[compat(...)].
    let mut compat_ident = Vec::with_capacity(data.fields.len());
    // Stores the target type for each of the legacy struct's fields.
    let mut type_defn: Vec<Type> = Vec::with_capacity(data.fields.len());
    // Stores the conversion function, if any, for each of the legacy struct's fields.
    let mut conv: Vec<Option<Path>> = Vec::with_capacity(data.fields.len());

    for field in data.fields.iter() {
        let ty = Some(&field.ty);
        let parsed_attr = match parse_field_compat_attr(&field.attrs, ty, &parsed_outer) {
            Err(err) => return format_compat_attr_err(err).into_compile_error(),
            Ok(parsed) => parsed,
        };

        compat_ident.push(parsed_attr.name
            .or_else(|| field.ident.clone())
            .expect(NAMED_FIELDS_ONLY)); // Only handle structs with named fields.
        type_defn.push(parsed_attr.ty
            .unwrap_or_else(|| field.ty.clone()));
        conv.push(parsed_attr.map_fn);
    }

    // Generate the compat-struct `into_compat` field conversions.
    let field_conv = {
        let converted_field: Vec<_> = data.fields
            .iter()
            .enumerate()
            .map(|(i, field)| {
                let ident = field.ident.as_ref().expect(NAMED_FIELDS_ONLY);
                const MSG: &'static str = "every field should have a `conv` item";
                let converted = match conv.get(i).expect(MSG) {
                    // eg. `Option::map(self.x, Into::into)`
                    // NOTE: Probably won't work for non- `Option::map` methods.
                    Some(conv) => quote_spanned! {field.span()=>
                        #conv(self.#ident, Into::into)
                    },
                    // eg. `self.x.into()`
                    None => quote_spanned! {field.span()=>
                        self.#ident.into()
                    },
                };

                // Move each field in `self` to its serialization helper type's corresponding
                // field, converting it if required.
                //
                // eg.
                // Orig   { x: i32, y: i32 }
                // Compat { a: i32, b: i32 }
                //
                // // return Compat { a: self.x, b: self.y }
                let compat = compat_ident.get(i);
                quote_spanned! {field.span()=>
                    #compat: #converted
                }
            })
            .collect();

        match &data.fields {
            Fields::Named(f) => quote_spanned! {f.span()=>
                { #( #converted_field, )* }
            },
            Fields::Unnamed(f) => quote_spanned! {f.span()=>
                ( #( #converted_field, )* )
            },
            Fields::Unit => proc_macro2::TokenStream::new(),
        }
    };

    let orig_struct_id = &ast.ident;
    let compat_struct_id = format_ident!("{COMPAT_PREFIX}{}", orig_struct_id);
    let generics = &ast.generics;
    let semi_token = data.semi_token;
    quote! {
        /// Semver-6.0.0 compatible serialization helper.
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        #[serde_with::skip_serializing_none] // I think this has appear before derive(Serialize).
        #[derive(serde::Serialize, Debug, Clone)]
        #[serde(rename_all = "snake_case")]
        pub(crate) struct #compat_struct_id #generics {
            #(#compat_ident: #type_defn),*
        } #semi_token

        #[automatically_derived]
        impl #orig_struct_id {
            /// Converts the legacy struct into its semver-6.0.0 compatible serialization helper
            /// type.
            pub fn into_compat(self) -> #compat_struct_id {
                #compat_struct_id #field_conv
            }
        }

        // Helper `From` implementation for the legacy struct -> semver-6.0.0 compatible struct.
        impl From<#orig_struct_id> for #compat_struct_id {
            fn from(value: #orig_struct_id) -> Self {
                value.into_compat()
            }
        }
    }
}

// ------------------------------------------------------------------------------------------------

/// Generates a semver-6.0.0 compatible enum for serialization purposes.
pub(crate) fn generate_compat_enum(ast: &DeriveInput, data: &DataEnum)
    -> proc_macro2::TokenStream
{
    let orig_enum_id = &ast.ident;
    let generics = &ast.generics;
    let compat_enum_id = format_ident!("{COMPAT_PREFIX}{}", orig_enum_id);

    let parsed_outer = match parse_outer_compat_attr(&ast.attrs) {
        Ok(parsed) => parsed,
        Err(err) => return err.into_compile_error(),
    };

    // Check if the enum is #[serde(untagged)].
    let untagged = match parse_serde_untagged(&ast.attrs) {
        Ok(untagged) => untagged,
        Err(err) => return err.into_compile_error(),
    };

    // (either `Serialize` if no explicit discriminant is encountered or `Serialize_repr` if an
    // explicit discriminant assignment _is_ encountered).
    let (mut ser_crate, mut ser_derive)  = (format_ident!("serde"), format_ident!("Serialize"));

    // Stores the generated compat enum's variant definitions.
    let mut compat_defn = Vec::with_capacity(data.variants.len());
    // Stores the original -> compat variant `into` conversions.
    let mut convert_defn = Vec::with_capacity(data.variants.len());

    for var in data.variants.iter() {
        if var.discriminant.is_some() {
            // Serialize the enum into its discriminant number representation if any variant has an
            // explicitly assigned discriminant.
            //
            // eg. `enum Repr { A, B, C = 123 }`
            (ser_crate, ser_derive) = (
                format_ident!("serde_repr"),
                format_ident!("Serialize_repr"),
            );
        }

        let orig_type: Option<&Type> = match &var.fields {
            Fields::Unit => None,
            Fields::Unnamed(fields) => {
                match fields.unnamed.len() {
                    1 => fields.unnamed.get(0)
                        .map(|f| &f.ty),
                    _ => {
                        let msg = "GenerateCompat does not support enum variants with multiple \
                                  fields";
                        return Error::new(var.span(), msg)
                            .into_compile_error();
                    },
                }
            }
            Fields::Named(_) => {
                let msg = "GenerateCompat does not support enums with struct-like variants.";
                return Error::new(var.span(), msg)
                    .into_compile_error();
            },
        };
        let parsed_attr = match parse_field_compat_attr(&var.attrs, orig_type, &parsed_outer) {
            Err(err) => return format_compat_attr_err(err).into_compile_error(),
            Ok(parsed) => parsed,
        };

        let var_id = &var.ident;
        let compat_ident = parsed_attr.name.unwrap_or_else(|| var_id.clone());
        // Construct the compat-enum variant definition and `into_compat` conversion.
        match (&parsed_attr.ty, &parsed_attr.map_fn) {
            (Some(ty), map_fn) => {
                compat_defn.push(quote_spanned! {ty.span()=>
                    #compat_ident(#ty)
                });

                let into = match map_fn {
                    // Map the original -> compat type.
                    Some(map_fn) => quote! {
                        #map_fn(x, Into::into)
                    },
                    // Call the `Into` implementation if no `map` was specified. Usually this will
                    // be a no-op (eg. `let x: i32 = 2.into();`).
                    None => quote! {
                        x.into()
                    },
                };
                let map_span = parsed_attr.span
                    .unwrap_or_else(|| ty.span());
                convert_defn.push(quote_spanned! {map_span=>
                    Self::#var_id(x) => #compat_enum_id::#compat_ident(#into)
                });
            },

            (None, Some(map)) => {
                // #[compat(map = ...)] defined but no type change specified.
                // This isn't really an error but may be indicative of one; may as well force the
                // caller to confront it.
                let msg = format!("\"{MAP}\"");
                return Error::new(map.span(), msg)
                    .into_compile_error();
            },

            (None, None) => {
                compat_defn.push(quote_spanned! {var_id.span()=>
                    #compat_ident
                });
                convert_defn.push(quote_spanned! {var.span()=>
                    Self::#var_id => #compat_enum_id::#compat_ident
                });
            },
        };
    }

    quote! {
        /// Semver-6.0.0 compatible serialization helper.
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        #[derive(#ser_crate::#ser_derive, Debug, Clone)]
        #[serde(rename_all = "snake_case")]
        #untagged
        pub(crate) enum #compat_enum_id #generics {
            #(#compat_defn),*
        }

        #[automatically_derived]
        impl #orig_enum_id {
            /// Converts the legacy enum into its semver-6.0.0 compatible serialization helper
            /// type.
            pub fn into_compat(self) -> #compat_enum_id {
                match self {
                    #(#convert_defn),*
                }
            }
        }

        // Helper `From` implementation for the legacy enum -> semver-6.0.0 compatible enum.
        impl From<#orig_enum_id> for #compat_enum_id {
            fn from(value: #orig_enum_id) -> Self {
                value.into_compat()
            }
        }
    }
}

// ------------------------------------------------------------------------------------------------

struct ParsedFieldAttr {
    span: Option<Span>,
    name: Option<Ident>,
    ty: Option<Type>,
    map_fn: Option<Path>,
}

fn format_compat_attr_err(err: Error) -> Error {
    let msg = format!("Failed to parse #[{COMPAT_ATTR}] args: {err}");
    Error::new(err.span(), msg)
}

/// Parses the #[compat(...)] helper attribute on struct fields or enum variants.
///
/// eg.
/// ```
/// #[derive(GenerateCompat)]
/// struct Foo {
///     #[compat(name = xyzzy)] // <<< Parses this
///     bar: i32,
/// }
/// ```
fn parse_field_compat_attr<'a, I>(attributes: I, orig_type: Option<&Type>, outer: &ParsedOuterAttr)
    -> Result<ParsedFieldAttr>
where
    I: IntoIterator<Item = &'a Attribute>,
{
    let mut span = None;
    let mut attr_name = None;
    let mut attr_type = None;
    let mut mapping = None;

    for attr in attributes.into_iter() {
        if attr.path() != COMPAT_ATTR {
            continue;
        }

        attr.parse_nested_meta(|meta| {
            // #[compat(name = foo)]
            if meta.path == NAME {
                let value = meta.value()?;
                span = Some(value.span());
                attr_name = Some(value.parse()?);

            // #[compat(type = Option<i32>)]
            } else if meta.path == TYPE {
                match orig_type {
                    None => {
                        let msg = format!("\"{TYPE}\" missing original field or enum variant \
                            type");
                        return Err(meta.error(msg));
                    },

                    Some(orig_type) => {
                        let value = meta.value()?;
                        span = Some(value.span());
                        let mut meta_type: Type = value.parse()?;
                        if let Some(placeholder) = outer.placeholder.as_ref() {
                            replace_compat_placeholder(
                                orig_type,
                                &mut meta_type,
                                placeholder)?;
                        }
                        attr_type = Some(meta_type);
                    },
                }

            // #[compat(map = Option::map)]
            } else if meta.path == MAP {
                let value = meta.value()?;
                span = Some(value.span());
                mapping = Some(value.parse()?);
            }
            Ok(())
        })?;
    }

    Ok(ParsedFieldAttr {
        span,
        name: attr_name,
        ty: attr_type,
        map_fn: mapping,
    })
}

struct ParsedOuterAttr {
    placeholder: Option<Ident>,
}

/// Parses the #[compat(...)] helper attribute on the outer struct or enum definition.
///
/// eg.
/// ```
/// #[derive(GenerateCompat)]
/// #[compat(placeholder = P)] // <<< Parses this
/// struct Foo {
///     bar: i32,
/// }
/// ```
fn parse_outer_compat_attr<'a, I>(attributes: I) -> Result<ParsedOuterAttr>
where
    I: IntoIterator<Item = &'a Attribute>,
{
    let mut placeholder = None;

    for ast_attr in attributes.into_iter() {
        if ast_attr.path() != COMPAT_ATTR {
            continue;
        }

        let parser = Punctuated::<Meta, Token![,]>::parse_terminated;
        let nested = ast_attr.parse_args_with(parser)?;
        for meta in nested.iter() {
            if meta.path() == PLACEHOLDER {
                match &meta.require_name_value()?.value {
                    Expr::Path(expr) => {
                        placeholder = expr.path.require_ident()?
                            .clone()
                            .into();
                    },

                    Expr::Lit(expr) => match &expr.lit {
                        Lit::Str(s) => {
                            placeholder = Some(s.parse()?);
                        },

                        lit => {
                            let msg = format!("unexpected \"{PLACEHOLDER}\": {lit:?}");
                            return Err({
                                let err = Error::new(meta.span(), msg);
                                format_compat_attr_err(err)
                            });
                        },
                    },

                    expr => {
                        let msg = format!("unexpected \"{PLACEHOLDER}\": {expr:?}");
                        return Err({
                            let err = Error::new(meta.span(), msg);
                            format_compat_attr_err(err)
                        });
                    },
                }
            }
        }
    }

    Ok(ParsedOuterAttr { placeholder })
}

/// Parses the original enum-level attributes for the `#[serde(untagged)]` attribute.
///
/// Returns a [`Result`] containing `Some("#[serde(untagged)]")` if found; `None` if no attribute
/// matches.
fn parse_serde_untagged<'a, I>(attributes: I) -> Result<Option<proc_macro2::TokenStream>>
where
    I: IntoIterator<Item = &'a Attribute>,
{
    let mut untagged = None;
    for ast_attr in attributes.into_iter() {
        if !ast_attr.path().is_ident("serde") {
            continue;
        }

        // `attr.parse_nested_meta` seems to fail if there is only a single expression (eg.
        // `#[serde(rename = "foo")]`). So we need to "manually" parse with `parse_args_with`.
        let parser = Punctuated::<Meta, Token![,]>::parse_terminated;
        let nested = ast_attr.parse_args_with(parser)?;
        for meta in nested.iter() {
            if meta.path().is_ident("untagged") {
                untagged = Some(quote_spanned! {ast_attr.span()=>
                    #[serde(untagged)]
                });
                break;
            }
        }

        if untagged.is_some() {
            break;
        }
    }
    Ok(untagged)
}
