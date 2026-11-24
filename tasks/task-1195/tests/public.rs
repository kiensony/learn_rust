use task_1195::*;

#[test]
fn public_examples() {
    assert_eq!(
        marks_removed_at_both_ends_hash("outside[red#blue]ignored", "backup"),
        "red#blue"
    );
    assert_eq!(
        marks_removed_at_both_ends_hash("[猫###a]", "backup"),
        "猫###a"
    );
}
