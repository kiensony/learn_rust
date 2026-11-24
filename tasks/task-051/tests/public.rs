mod common;

#[test]
fn examples() {
    common::check(&["Bus"], "Arriving: Bus\n", "");
    common::check(&["Line 4"], "Arriving: Line 4\n", "");
}
