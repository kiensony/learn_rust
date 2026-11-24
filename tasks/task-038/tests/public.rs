mod common;

#[test]
fn examples() {
    common::check(&[], "ready\n", "");
    common::check(&["unused"], "ready\n", "");
}
