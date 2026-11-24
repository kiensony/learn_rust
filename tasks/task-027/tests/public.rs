mod common;

#[test]
fn examples() {
    common::check(&["Board now"], "Board now\n", "");
    common::check(&["23%"], "23%\n", "");
}
