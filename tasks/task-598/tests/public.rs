use task_598::*;

#[test]
fn public_examples() {
    assert_eq!(closing_field_hash("outside[red#blue]ignored"), "blue");
    assert_eq!(closing_field_hash("[猫###a]"), "a");
}
