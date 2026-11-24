use task_1194::*;

#[test]
fn public_examples() {
    assert_eq!(
        closing_marks_removed_hash("outside[red#blue]ignored", "backup"),
        "red#blue"
    );
    assert_eq!(closing_marks_removed_hash("[猫###a]", "backup"), "猫###a");
}
