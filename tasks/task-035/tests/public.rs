mod common;

#[test]
fn examples() {
    common::check(&[], "Counts seats on the ferry\n", "");
    common::check(&["unused"], "Counts seats on the ferry\n", "");
}
