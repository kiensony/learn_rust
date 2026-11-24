mod common;

#[test]
fn examples() {
    common::check(&["enter", "Mina"], "Mina <- enter\n", "");
    common::check(&["wait", "Bo"], "Bo <- wait\n", "");
}
