mod common;

#[test]
fn examples() {
    common::check(&[], "North gate\nOpen\n", "");
    common::check(&["unused"], "North gate\nOpen\n", "");
}
