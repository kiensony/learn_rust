mod common;

#[test]
fn examples() {
    common::check(&[], "open {file}\n", "");
    common::check(&["unused"], "open {file}\n", "");
}
