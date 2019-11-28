use trybuild::TestCases;

#[test]
fn safety() {
    let t = TestCases::new();
    t.compile_fail("tests/safety/*.rs");
}
