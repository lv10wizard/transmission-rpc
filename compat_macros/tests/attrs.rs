use compat_macros::SemverCompat;
use semver::Version;
use serde::Serialize;

mod ui;

#[cfg(test)]
mod serialize_added_struct {
    use super::*;
    use test_case::test_case;

    #[serde_with::skip_serializing_none]
    #[derive(SemverCompat, Serialize)]
    #[serde(rename_all = "camelCase")]
    struct CompatAdded {
        #[added = "2.0.0"]
        added_field: Option<i32>,
    }

    #[test_case(
        CompatAdded { added_field: Some(3) }, Version::new(1, 3, 0)
        => r#"{}"#
        ; "before added version"
    )]
    #[test_case(
        CompatAdded { added_field: Some(3) }, Version::new(2, 0, 0)
        => r#"{"addedField":3}"#
        ; "eq added version"
    )]
    #[test_case(
        CompatAdded { added_field: Some(3) }, Version::new(5, 0, 0)
        => r#"{"addedField":3}"#
        ; "after added version"
    )]
    #[test_case(
        CompatAdded { added_field: Some(3) }, Version::new(6, 0, 0)
        => r#"{"added_field":3}"#
        ; "6.0.0 added snake_case"
    )]

    fn serialize(data: CompatAdded, version: Version) -> String {
        let compat = data.into_compat(&version)
            .expect("compat version should exist");
        serde_json::to_string(&compat)
            .expect("data should serialize")
    }
}

#[cfg(test)]
mod serialize_added_enum {
    use super::*;
    use test_case::test_case;

    #[derive(SemverCompat, Serialize)]
    #[serde(rename_all = "camelCase")]
    enum CompatAdded {
        #[added = "2.0.0"]
        AddedField,
    }

    #[test_case(
        CompatAdded::AddedField, Version::new(1, 3, 0)
        => "\"\""
        ; "before added version"
    )]
    #[test_case(
        CompatAdded::AddedField, Version::new(2, 0, 0)
        => r#""addedField""#
        ; "eq added version"
    )]
    #[test_case(
        CompatAdded::AddedField, Version::new(5, 0, 0)
        => r#""addedField""#
        ; "after added version"
    )]
    #[test_case(
        CompatAdded::AddedField, Version::new(6, 0, 0)
        => r#""added_field""#
        ; "6.0.0 added snake_case"
    )]

    fn serialize(data: CompatAdded, version: Version) -> String {
        let compat = data.into_compat(&version)
            .expect("compat version should exist");
        match serde_json::to_string(&compat) {
            Ok(json) => json,
            Err(err) => {
                println!("serialize failed: {err}");
                String::default()
            },
        }
    }
}

#[cfg(test)]
mod serialize_removed_struct {
    use super::*;
    use test_case::test_case;

    #[serde_with::skip_serializing_none]
    #[derive(SemverCompat, Serialize)]
    #[serde(rename_all = "camelCase")]
    struct CompatRemoved {
        #[removed = "5.0.0"]
        removed_field: Option<i32>,
    }

    #[test_case(
        CompatRemoved { removed_field: Some(3) }, Version::new(3, 3, 0)
        => r#"{"removedField":3}"#
        ; "before removed version"
    )]
    #[test_case(
        CompatRemoved { removed_field: Some(3) }, Version::new(5, 0, 0)
        => r#"{}"#
        ; "eq removed version"
    )]
    #[test_case(
        CompatRemoved { removed_field: Some(3) }, Version::new(5, 0, 0)
        => r#"{}"#
        ; "after removed version"
    )]

    fn serialize(data: CompatRemoved, version: Version) -> String {
        let compat = data.into_compat(&version)
            .expect("compat version should exist");
        serde_json::to_string(&compat)
            .expect("data should serialize")
    }
}

#[cfg(test)]
mod serialize_removed_enum {
    use super::*;
    use test_case::test_case;

    #[derive(SemverCompat, Serialize)]
    #[serde(rename_all = "camelCase")]
    enum CompatRemoved {
        #[removed = "5.0.0"]
        AddedField,
    }

    #[test_case(
        CompatRemoved::AddedField, Version::new(3, 3, 0)
        => r#""addedField""#
        ; "before removed version"
    )]
    #[test_case(
        CompatRemoved::AddedField, Version::new(5, 0, 0)
        => "\"\""
        ; "eq removed version"
    )]
    #[test_case(
        CompatRemoved::AddedField, Version::new(6, 0, 0)
        => "\"\""
        ; "after removed version"
    )]

    fn serialize(data: CompatRemoved, version: Version) -> String {
        let compat = data.into_compat(&version)
            .expect("compat version should exist");
        match serde_json::to_string(&compat) {
            Ok(json) => json,
            Err(err) => {
                println!("serialize failed: {err}");
                String::default()
            },
        }
    }
}

#[cfg(test)]
mod serialize_renamed_struct {
    use super::*;
    use test_case::test_case;

    #[serde_with::skip_serializing_none]
    #[derive(SemverCompat, Serialize)]
    #[serde(rename_all = "camelCase")]
    struct CompatRenamed {
        #[renamed = r#"("3.3.0", "foo_bar")"#]
        renamed_field: Option<i32>,
    }

    #[test_case(
        CompatRenamed { renamed_field: Some(3) }, Version::new(2, 0, 0)
        => r#"{"renamedField":3}"#
        ; "before renamed version"
    )]
    #[test_case(
        CompatRenamed { renamed_field: Some(3) }, Version::new(3, 3, 0)
        => r#"{"fooBar":3}"#
        ; "eq renamed version"
    )]
    #[test_case(
        CompatRenamed { renamed_field: Some(3) }, Version::new(5, 0, 0)
        => r#"{"fooBar":3}"#
        ; "after renamed version"
    )]
    #[test_case(
        CompatRenamed { renamed_field: Some(3) }, Version::new(6, 0, 0)
        => r#"{"foo_bar":3}"#
        ; "6.0.0 renamed snake_case"
    )]

    fn serialize(data: CompatRenamed, version: Version) -> String {
        let compat = data.into_compat(&version)
            .expect("compat version should exist");
        serde_json::to_string(&compat)
            .expect("data should serialize")
    }
}

#[cfg(test)]
mod serialize_renamed_enum {
    use super::*;
    use test_case::test_case;

    #[derive(SemverCompat, Serialize)]
    #[serde(rename_all = "camelCase")]
    enum CompatRenamed {
        #[renamed = r#"("3.1.0", "NewName")"#]
        RenamedField,
    }

    #[test_case(
        CompatRenamed::RenamedField, Version::new(2, 0, 0)
        => r#""renamedField""#
        ; "before renamed version"
    )]
    #[test_case(
        CompatRenamed::RenamedField, Version::new(3, 1, 0)
        => r#""newName""#
        ; "eq renamed version"
    )]
    #[test_case(
        CompatRenamed::RenamedField, Version::new(5, 0, 0)
        => r#""newName""#
        ; "after renamed version"
    )]
    #[test_case(
        CompatRenamed::RenamedField, Version::new(6, 0, 0)
        => r#""new_name""#
        ; "6.0.0 renamed snake_case"
    )]

    fn serialize(data: CompatRenamed, version: Version) -> String {
        let compat = data.into_compat(&version)
            .expect("compat version should exist");
        match serde_json::to_string(&compat) {
            Ok(json) => json,
            Err(err) => {
                println!("serialize failed: {err}");
                String::default()
            },
        }
    }
}

#[cfg(test)]
mod serialize_inner_compat_enum {
    use super::*;
    use test_case::test_case;

    #[serde_with::skip_serializing_none]
    #[derive(SemverCompat, Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Outer {
        #[compat]
        outer_field: Option<Vec<I>>,
    }

    #[derive(SemverCompat, Serialize)]
    #[serde(rename_all = "camelCase")]
    enum I {
        #[renamed = r#"("2.0.0", "FooBar")"#]
        FirstVar,
        #[removed = "3.0.0"]
        SecondVar,
        #[added = "6.0.0"]
        ThirdVar,
    }

    #[test_case(
        Outer {
            outer_field: Some(vec![I::FirstVar, I::SecondVar, I::ThirdVar]),
        }, Version::new(1, 3, 0)
        => r#"{"outerField":["firstVar","secondVar",""]}"#
        ; "before renamed version"
    )]
    #[test_case(
        Outer {
            outer_field: Some(vec![I::FirstVar, I::SecondVar, I::ThirdVar]),
        }, Version::new(2, 0, 0)
        => r#"{"outerField":["fooBar","secondVar",""]}"#
        ; "eq renamed version"
    )]
    #[test_case(
        Outer {
            outer_field: Some(vec![I::FirstVar, I::SecondVar, I::ThirdVar]),
        }, Version::new(2, 1, 0)
        => r#"{"outerField":["fooBar","secondVar",""]}"#
        ; "after renamed before removed version"
    )]
    #[test_case(
        Outer {
            outer_field: Some(vec![I::FirstVar, I::SecondVar, I::ThirdVar]),
        }, Version::new(3, 0, 0)
        => r#"{"outerField":["fooBar","",""]}"#
        ; "eq removed version"
    )]
    #[test_case(
        Outer {
            outer_field: Some(vec![I::FirstVar, I::SecondVar, I::ThirdVar]),
        }, Version::new(5, 0, 0)
        => r#"{"outerField":["fooBar","",""]}"#
        ; "after removed before added version"
    )]
    #[test_case(
        Outer {
            outer_field: Some(vec![I::FirstVar, I::SecondVar, I::ThirdVar]),
        }, Version::new(6, 0, 0)
        => r#"{"outer_field":["foo_bar","","third_var"]}"#
        ; "eq added version"
    )]
    #[test_case(
        Outer {
            outer_field: Some(vec![I::FirstVar, I::SecondVar, I::ThirdVar]),
        }, Version::new(6, 0, 1)
        => r#"{"outer_field":["foo_bar","","third_var"]}"#
        ; "after added version"
    )]

    fn serialize(data: Outer, version: Version) -> String {
        let compat = data.into_compat(&version)
            .expect("compat version should exist");
        serde_json::to_string(&compat)
            .expect("data should serialize")
    }
}

#[cfg(test)]
mod serialize_inner_compat_struct {
    use super::*;
    use test_case::test_case;

    #[serde_with::skip_serializing_none]
    #[derive(SemverCompat, Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Outer {
        #[compat]
        outer_field: Option<I>,
    }

    #[serde_with::skip_serializing_none]
    #[derive(SemverCompat, Serialize)]
    #[serde(rename_all = "camelCase")]
    struct I {
        #[renamed = r#"("2.0.0", "my_renamed_var")"#]
        first_var: Option<i32>,
        #[removed = "3.0.0"]
        second_var: Option<String>,
        #[added = "6.0.0"]
        third_var: Option<f32>,
    }

    #[test_case(
        Outer {
            outer_field: Some(I {
                first_var: Some(6),
                second_var: Some("two".to_string()),
                third_var: Some(1.23),
            }),
        }, Version::new(1, 3, 0)
        => r#"{"outerField":{"firstVar":6,"secondVar":"two"}}"#
        ; "before renamed version"
    )]
    #[test_case(
        Outer {
            outer_field: Some(I {
                first_var: Some(6),
                second_var: Some("two".to_string()),
                third_var: Some(1.23),
            }),
        }, Version::new(2, 0, 0)
        => r#"{"outerField":{"myRenamedVar":6,"secondVar":"two"}}"#
        ; "eq renamed version"
    )]
    #[test_case(
        Outer {
            outer_field: Some(I {
                first_var: Some(6),
                second_var: Some("two".to_string()),
                third_var: Some(1.23),
            }),
        }, Version::new(2, 1, 0)
        => r#"{"outerField":{"myRenamedVar":6,"secondVar":"two"}}"#
        ; "after renamed before removed version"
    )]
    #[test_case(
        Outer {
            outer_field: Some(I {
                first_var: Some(6),
                second_var: Some("two".to_string()),
                third_var: Some(1.23),
            }),
        }, Version::new(3, 0, 0)
        => r#"{"outerField":{"myRenamedVar":6}}"#
        ; "eq removed version"
    )]
    #[test_case(
        Outer {
            outer_field: Some(I {
                first_var: Some(6),
                second_var: Some("two".to_string()),
                third_var: Some(1.23),
            }),
        }, Version::new(5, 0, 0)
        => r#"{"outerField":{"myRenamedVar":6}}"#
        ; "after removed before added version"
    )]
    #[test_case(
        Outer {
            outer_field: Some(I {
                first_var: Some(6),
                second_var: Some("two".to_string()),
                third_var: Some(1.23),
            }),
        }, Version::new(6, 0, 0)
        => r#"{"outer_field":{"my_renamed_var":6,"third_var":1.23}}"#
        ; "eq added version"
    )]
    #[test_case(
        Outer {
            outer_field: Some(I {
                first_var: Some(6),
                second_var: Some("two".to_string()),
                third_var: Some(1.23),
            }),
        }, Version::new(6, 0, 1)
        => r#"{"outer_field":{"my_renamed_var":6,"third_var":1.23}}"#
        ; "after added version"
    )]

    fn serialize(data: Outer, version: Version) -> String {
        let compat = data.into_compat(&version)
            .expect("compat version should exist");
        serde_json::to_string(&compat)
            .expect("data should serialize")
    }
}
