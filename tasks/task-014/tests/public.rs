mod common;

#[test]
fn examples() {
    common::check(&[], "Dear Jo,\n\nSee you soon.\n", "");
    common::check(&["unused"], "Dear Jo,\n\nSee you soon.\n", "");
}
