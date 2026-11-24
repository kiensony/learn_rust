mod common;

#[test]
fn examples() {
    common::check(&[], "Parcel accepted.\n", "");
    common::check(&["unused"], "Parcel accepted.\n", "");
}
