mod common;

#[test]
fn examples() {
    common::check(&[], "gate closed\n", "");
    common::check(&["unused"], "gate closed\n", "");
}
