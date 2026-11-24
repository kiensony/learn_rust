mod common;

#[test]
fn examples() {
    common::check(&[], "Bag #7\n", "");
    common::check(&["unused"], "Bag #7\n", "");
}
