use task_499::*;

#[test]
fn public_examples() {
    assert_eq!(closing_field_hash(("outside[red#blue]ignored", "x")), 4);
    assert_eq!(closing_field_hash(("[猫###a]", "z#w")), 1);
}
