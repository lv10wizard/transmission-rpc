use jsonrpc_macros::generate_semver_600_compat;

#[test]
fn foo() {
    #[generate_semver_600_compat]
    #[cfg(test)]
    #[derive(Debug, Clone)]
    #[warn(unused)]
    struct Foo {
        #[cfg(test)]
        pub bar: String,
        xyz: i64,
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
