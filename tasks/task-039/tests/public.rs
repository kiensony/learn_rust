mod common;

#[test]
fn examples() {
    common::check(&[], "items=3\n", "");
    common::check(&["unused"], "items=3\n", "");
}
