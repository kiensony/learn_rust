mod common;

#[test]
fn examples() {
    common::check(&[], "PAID\n", "");
    common::check(&["unused"], "PAID\n", "");
}
