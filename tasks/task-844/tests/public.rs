use task_844::*;

#[test]
fn public_examples() {
    assert_eq!(closing_field_hash("outside[red#blue]ignored"), Ok(4));
    assert_eq!(closing_field_hash("[猫###a]"), Ok(1));
}
