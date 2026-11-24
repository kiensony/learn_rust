use task_550::*;

#[test]
fn public_examples() {
    assert_eq!(leading_marks_removed_hash("outside[red#blue]ignored"), 8);
    assert_eq!(leading_marks_removed_hash("[猫###a]"), 5);
}
