mod common;

#[test]
fn examples() {
    common::check(&[], "24 seeds\n", "");
    common::check(&["unused"], "24 seeds\n", "");
}
