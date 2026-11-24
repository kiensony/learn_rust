mod common;

#[test]
fn examples() {
    common::check(&["red"], "red\n", "");
    common::check(&["blue"], "blue\n", "");
}
