use task_1193::*;

#[test]
fn public_examples() {
    assert_eq!(
        leading_marks_removed_hash("outside[red#blue]ignored", "backup"),
        "red#blue"
    );
    assert_eq!(leading_marks_removed_hash("[猫###a]", "backup"), "猫###a");
}
