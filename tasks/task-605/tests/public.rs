use task_605::*;

#[test]
fn public_examples() {
    assert_eq!(
        longest_field_with_last_tie_hash("outside[red#blue]ignored"),
        "blue"
    );
    assert_eq!(longest_field_with_last_tie_hash("[猫###a]"), "猫");
}
