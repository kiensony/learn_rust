mod common;

#[test]
fn examples() {
    common::check(&[], "Name> ", "");
    common::check(&["unused"], "Name> ", "");
}
