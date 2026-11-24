use task_600::*;

#[test]
fn public_examples() {
    assert_eq!(
        closing_marks_removed_hash("outside[red#blue]ignored"),
        "red#blue"
    );
    assert_eq!(closing_marks_removed_hash("[猫###a]"), "猫###a");
}
