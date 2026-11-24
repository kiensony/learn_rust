use task_1198::*;

#[test]
fn public_examples() {
    assert_eq!(
        first_nonempty_field_hash("outside[red#blue]ignored", "backup"),
        "red"
    );
    assert_eq!(first_nonempty_field_hash("[猫###a]", "backup"), "猫");
}
