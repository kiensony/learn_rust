mod common;

#[test]
fn examples() {
    common::check(&[], "42\n", "using fallback\n");
    common::check(&["unused"], "42\n", "using fallback\n");
}
