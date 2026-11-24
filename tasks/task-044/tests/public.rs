mod common;

#[test]
fn examples() {
    common::check(&["West"], "Room: West\n", "");
    common::check(&["Blue"], "Room: Blue\n", "");
}
