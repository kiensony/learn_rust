mod common;

#[test]
fn examples() {
    common::check(&[], "Beacon on\n", "");
    common::check(&["unused"], "Beacon on\n", "");
}
