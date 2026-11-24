mod common;

#[test]
fn examples() {
    common::check(&[], "Dock three\n", "");
    common::check(&["unused"], "Dock three\n", "");
}
