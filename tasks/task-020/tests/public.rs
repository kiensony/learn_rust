mod common;

#[test]
fn examples() {
    common::check(&[], "Platform 4\n", "");
    common::check(&["unused"], "Platform 4\n", "");
}
