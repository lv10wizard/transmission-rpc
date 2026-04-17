use compat_macros::SemverCompat;

#[derive(SemverCompat, serde::Serialize)]
struct DuplicateAttrAdded {
    #[added = "1.3.0"]
    #[added = "5.3.0"]
    foo_bar: Option<String>,
}

#[derive(SemverCompat, serde::Serialize)]
struct DuplicateAttrRemoved {
    #[removed = "1.3.0"]
    #[removed = "2.0.0"]
    foo_bar: Option<String>,
}

#[derive(SemverCompat, serde::Serialize)]
struct DuplicateAttrRenamed {
    #[renamed = r#"("2.0.0", "lorem_ipsum")"#]
    #[renamed = r#"("3.0.0", "bar_baz")"#]
    foo_bar: Option<String>,
}

#[derive(SemverCompat, serde::Serialize)]
struct DuplicateVersionAddedRemoved {
    #[added = "6.0.0"]
    #[removed = "6.0.0"]
    foo_bar: Option<i32>,
}

#[derive(SemverCompat, serde::Serialize)]
struct DuplicateVersionAddedRenamed {
    #[added = "5.0.0"]
    #[renamed = r#"("5.0.0", "peer_id")"#]
    foo_bar: Option<i32>,
}

#[derive(SemverCompat, serde::Serialize)]
struct DuplicateVersionRemovedRenamed {
    #[removed = "2.0.0"]
    #[renamed = r#"("2.0.0", "peer_port")"#]
    foo_bar: Option<i32>,
}

#[derive(SemverCompat, serde::Serialize)]
struct DuplicateVersionAddedRemovedRenamed {
    #[added = "6.0.1"]
    #[removed = "6.0.1"]
    #[renamed = r#"("6.0.1", "holy_toledo")"#]
    foo_bar: Option<i32>,
}

fn main() {}
