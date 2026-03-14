//! This file defines `#[derive(GenerateCompat)]` usages for debugging with [`cargo-expand`].
//!
//! How I use this:
//!
//! * Edit macro code: `compat_macros/lib.rs`
//!
//! * cargo expand --test semver_600_test
//!
//! [`cargo-expand`]: <https://github.com/dtolnay/cargo-expand>

use compat_macros::GenerateCompat;
use serde::Serialize;

#[test]
fn compat_replace_struct_field() {
    let x: Option<i32> = Some(8);
    let y: Option<i64> = Option::map(x, Into::into);

    #[derive(GenerateCompat, Serialize, Debug)] // Are Serialize, Debug included in `.attrs`?
    #[serde(rename_all = "snake_case")] // Are derive-helper attributes included?
    struct Foo {
        #[compat_name(xyz)]
        #[compat_type(Option<i64>, convert_with = "Option::map")]
        abc: Option<i32>,
    }
}

#[test]
fn compat_replace_tuple_struct() {
    #[derive(GenerateCompat, Serialize, Debug)]
    #[serde(rename_all = "kebab-case")]
    struct Bar(
        #[compat_type(u16)]
        u8,
        #[compat_type(i32)]
        i8,
        #[compat_type(Option<i32>)]
        Option<i8>,
    );
}
