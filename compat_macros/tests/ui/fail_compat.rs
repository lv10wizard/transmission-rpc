use compat_macros::SemverCompat;
use serde::Serialize;

#[derive(SemverCompat, Serialize)]
enum CompatMissingType {
    #[compat]
    FooBar,
}

fn main() {}
