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
    #[derive(GenerateCompat, Serialize, Debug)] // Are Serialize, Debug included in `.attrs`?
    #[serde(rename_all = "snake_case")] // Are derive-helper attributes included?
    struct _Foo {
        #[compat(name = xyz, type = Option<i64>, map = Option::map)]
        abc: Option<i32>,

        def: String,

        #[compat(type = u16)]
        zzz: u8,
    }
}
