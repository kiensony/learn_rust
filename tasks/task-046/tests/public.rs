mod common;

#[test]
fn examples() {
    common::check(&["A12"], "A12\n", "");
    common::check(&["B34", "C56"], "B34\n", "");
}
