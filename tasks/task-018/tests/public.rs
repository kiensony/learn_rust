mod common;

#[test]
fn examples() {
    common::check(&[], "C:\\course\\notes\n", "");
    common::check(&["unused"], "C:\\course\\notes\n", "");
}
