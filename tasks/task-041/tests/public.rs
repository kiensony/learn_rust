mod common;

#[test]
fn examples() {
    common::check(&[], "name\tscore\n", "");
    common::check(&["unused"], "name\tscore\n", "");
}
