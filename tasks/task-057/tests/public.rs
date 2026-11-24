mod common;

#[test]
fn examples() {
    common::check(&["123"], "123\n", "receipt copied\n");
    common::check(&["A 7"], "A 7\n", "receipt copied\n");
}
