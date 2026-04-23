use compat_macros::SemverCompat;
use serde::Serialize;

#[derive(SemverCompat, Serialize)]
struct UnknownVersionAdded {
    #[added = "0.0.0"]
    uknown_version: i32,
}

#[derive(SemverCompat, Serialize)]
enum UnknownVersionRemoved {
    #[removed = "0.0.0"]
    UknownVersion,
}

#[derive(SemverCompat, Serialize)]
struct UnknownVersionRenamed {
    #[renamed = r#"("0.0.0", "ShouldFail")"#]
    unknown_version: Option<String>,
}

fn main() {}
