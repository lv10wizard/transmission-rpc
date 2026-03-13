use jsonrpc_macros::{compat_replace, GenerateCompat};

#[test]
fn foo() {
    #[generate_semver_600_compat]
    #[cfg(not(feature = "foo"))]
    #[derive(Debug, Clone)]
    #[warn(unused)]
    pub(crate) struct Foo {
        #[cfg(test)]
        pub bar: String,
        xyz: Option<Vec<String>>,
    }

    assert!(true);
}

#[test]
fn bar() {
    #[generate_semver_600_compat]
    #[cfg(test)]
    #[derive(Debug, Clone)]
    struct UnitStruct;

    assert!(true);
}

#[test]
fn compat_replace_struct_field() {
    #[derive(GenerateCompat)]
    struct Foo {
        #[compat_name("xyz")]
        #[compat_type("f32")]
        abc: Option<i32>,
    }
}
