mod common;

#[test]
fn examples() {
    common::check(&[], "Pier 8\n", "");
    common::check(&["unused"], "Pier 8\n", "");
}
