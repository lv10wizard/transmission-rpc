#[test]
fn should_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/fail_compat.rs");
    t.compile_fail("tests/ui/fail_duplicate.rs");
    t.compile_fail("tests/ui/fail_version.rs");
}
