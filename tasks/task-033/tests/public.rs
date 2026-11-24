mod common;

#[test]
fn examples() {
    common::check(&[], "Gates open at nine.\n", "");
    common::check(&["unused"], "Gates open at nine.\n", "");
}
