use task_599::*;

#[test]
fn public_examples() {
    assert_eq!(
        leading_marks_removed_hash("outside[red#blue]ignored"),
        "red#blue"
    );
    assert_eq!(leading_marks_removed_hash("[猫###a]"), "猫###a");
}
