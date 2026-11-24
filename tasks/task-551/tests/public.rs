use task_551::*;

#[test]
fn public_examples() {
    assert_eq!(closing_marks_removed_hash("outside[red#blue]ignored"), 8);
    assert_eq!(closing_marks_removed_hash("[猫###a]"), 5);
}
