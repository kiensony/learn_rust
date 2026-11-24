use task_1199::*;

#[test]
fn public_examples() {
    assert_eq!(
        longest_field_with_last_tie_hash("outside[red#blue]ignored", "backup"),
        "blue"
    );
    assert_eq!(longest_field_with_last_tie_hash("[猫###a]", "backup"), "猫");
}
