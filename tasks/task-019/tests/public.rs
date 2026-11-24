mod common;

#[test]
fn examples() {
    common::check(&[], "begin\nend\n", "");
    common::check(&["unused"], "begin\nend\n", "");
}
