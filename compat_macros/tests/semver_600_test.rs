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
        abc: String,

        #[compat(name = xyz, type = Option<i64>, map = Option::map)]
        def: Option<i8>,

        #[compat(type = u16)]
        zzz: u8,
    }

    #[derive(GenerateCompat)]
    #[compat(placeholder = PLACEHOLDER)]
    struct _Bar {
        #[compat(type = Option<PLACEHOLDER>, map = Option::map)]
        foo: Option<_Foo>,
    }
}

#[test]
fn compat_replace_enum_unit() {
    #[derive(GenerateCompat, Serialize)]
    #[serde(untagged)]
    #[compat(placeholder = P)]
    enum _Lorem {
        #[compat(type = P)]
        A(_Ipsum),
    }

    #[derive(GenerateCompat, Serialize)]
    enum _Ipsum {
        B,
    }
}
