//! This file defines `#[derive(SemverCompat)]` usages for debugging with [`cargo-expand`].
//!
//! How I use this:
//!
//! * Edit macro code: `compat_macros/lib.rs`
//!
//! * cargo expand --test semver_600_test
//!
//! [`cargo-expand`]: <https://github.com/dtolnay/cargo-expand>

use compat_macros::SemverCompat;
use serde::Serialize;

#[test]
fn compat_replace_struct_field() {
    #[serde_with::skip_serializing_none]
    #[derive(SemverCompat, Serialize, Debug)] // Are Serialize, Debug included in `.attrs`?
    #[serde(rename_all = "snake_case")] // Are derive-helper attributes included?
    struct _Foo {
        abc: String,

        #[renamed(semver = "5.2.0", name = "xyz")]
        def: Option<i8>,

        #[added(semver = "6.0.0")]
        zzz: Option<u8>,
    }

    #[derive(SemverCompat)]
    struct _Bar {
        #[compat(type = Option<_>)]
        foo: Option<_Foo>,
    }
}

#[test]
fn compat_replace_enum_unit() {
    #[derive(SemverCompat, Serialize)]
    #[serde(untagged)]
    enum _Lorem {
        #[compat(type = _)]
        A(_Ipsum),
    }

    #[derive(SemverCompat, Serialize)]
    enum _Ipsum {
        B,
    }
}
