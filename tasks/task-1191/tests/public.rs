use task_1191::*;

#[test]
fn public_examples() {
    assert_eq!(
        leading_field_hash("outside[red#blue]ignored", "backup"),
        "red"
    );
    assert_eq!(leading_field_hash("[猫###a]", "backup"), "猫");
}
