use std::collections::HashMap;

use quote::quote;
use semver::Version;
use syn::{DataEnum, DataStruct, Fields, Ident, LitStr, Result};

use crate::{compat::{FieldOrVar, Kind, parse_attr}};

/// Generates tokens to implement [`serde::Serialize`].
pub(crate) trait SerializeImpl {
    fn to_serialize_impl(&self, ident: &Ident)
        -> Result<proc_macro2::TokenStream>;
}

impl SerializeImpl for DataEnum {
    fn to_serialize_impl(&self, ident: &Ident)
        -> Result<proc_macro2::TokenStream>
    {
        let enum_id_str = format!("{ident}");
        for variant in  self.variants.iter() {
            // TODO
            match &variant.fields {
                Fields::Unit => {},
                Fields::Unnamed(fields) => {},
                Fields::Named(_) => {},
            }
        }

        Ok(quote! {}) // TODO
    }
}

impl SerializeImpl for DataStruct {
    fn to_serialize_impl(&self, ident: &Ident)
        -> Result<proc_macro2::TokenStream>
    {
        let struct_id_str = format!("{ident}");
        let mut field_is_serialized = Vec::with_capacity(self.fields.len());
        let mut serialize_field = Vec::with_capacity(self.fields.len());
        for fv in self.fields.iter().map(FieldOrVar::from) {
            let field_id = fv.require_ident()?;
            let field_id_str = format!("{field_id}");
            let is_serialized = is_serialized_tokens(fv)?;

            serialize_field.push(quote! {
                match #is_serialized {
                    true => serializer.serialize_field(#field_id_str, &self.base.#field_id),
                    false => serializer.skip_field(#field_id_str),
                }
            });
            field_is_serialized.push(is_serialized);
        }

        // Assumptions:
        //     * `serializer` is the variable name of the Serializer generic type.
        //     * `self` is a struct with fields:
        //         - version: semver::Version
        //         - base: <the source-defined struct>
        Ok(quote! {
            let mut num_fields = 0;
            #(
                num_fields += match #field_is_serialized {
                    true => 1,
                    false => 0,
                };
            )*

            let mut serializer = serializer.serialize_struct(#struct_id_str, num_fields);
            #( #serialize_field )*
            serializer.end();
        })
    }
}

// -------------------------------------------------------------------------------------------------

/// Generates tokens to compare `version` against `self.version` returning `true` based on `op`
/// (less than or greater than).
fn is_included_tokens(version: &Version, op: proc_macro2::TokenStream)
    -> proc_macro2::TokenStream
{
    let (major, minor, patch) = (version.major, version.minor, version.patch);
    // We need to handle each semver part individually.
    // eg. `2.0.0 > 1.2.3`.
    //     2 >= 1 && 0 >= 2 && 0 >= 3
    //     ^^^^^^    ^^^^^^    ^^^^^^
    //        |         \________/
    //       true   &&   false => false
    let major_ok = quote! { ( self.version.major #op #major ) };
    let minor_ok = quote! { ( self.version.major == #major && self.version.minor #op #minor ) };
    let patch_ok = quote! {(
        self.version.major == #major
            && self.version.minor == #minor
            && self.version.patch #op= #patch // FIXME? does `#op=` emit a space between?
    )};

    quote! { ( #major_ok || #minor_ok || #patch_ok ) }
}

fn is_serialized_tokens(fv: FieldOrVar<'_>) -> Result<proc_macro2::TokenStream> {
    let compat_data = parse_attr(fv)?;
    let mut conditions = Vec::with_capacity(compat_data.changes.len());

    for (version, kind) in compat_data.changes.iter() {
        conditions.push(match kind {
            Kind::Added => is_included_tokens(version, quote! { > }),
            Kind::Removed => is_included_tokens(version, quote! { < }),
            Kind::Renamed(_) => continue,
        });
        conditions.push(quote! { && });
    }

    // Check for any of the following #[serde] attributes:
    //     * #[serde(skip)],
    //     * #[serde(skip_serializing)], or
    //     * #[serde(skip_serializing_if = "...")]
    for attr in fv.attributes() {
        if !attr.path().is_ident("serde") {
            continue;
        }

        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("skip") || meta.path.is_ident("skip_serializing") {
                conditions.push(quote! { false });
                conditions.push(quote! { && });

            } else if meta.path.is_ident("skip_serializing_if") {
                let value = meta.value()?;
                let path: LitStr = value.parse()?;
                let field_id = fv.require_ident()?;

                // Assumption: `self.base` refers to the source-defined struct
                //           | (`skip_serializing_if` only applies to struct fields).
                conditions.push(quote! { #path(&self.base.#field_id) });
                conditions.push(quote! { && });
            }

            Ok(())
        })?;
    }

    // Remove the trailing `&&`.
    _ = conditions.pop();

    Ok(quote! { #( #conditions )* })
}
