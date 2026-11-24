use task_548::*;

#[test]
fn public_examples() {
    assert_eq!(leading_field_hash("outside[red#blue]ignored"), 3);
    assert_eq!(leading_field_hash("[猫###a]"), 1);
}
