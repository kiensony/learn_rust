mod common;

#[test]
fn examples() {
    common::check(&[], "0.2.1\n", "");
    common::check(&["unused"], "0.2.1\n", "");
}
