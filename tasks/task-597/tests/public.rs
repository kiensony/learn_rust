use task_597::*;

#[test]
fn public_examples() {
    assert_eq!(leading_field_hash("outside[red#blue]ignored"), "red");
    assert_eq!(leading_field_hash("[猫###a]"), "猫");
}
