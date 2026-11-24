use task_895::*;

#[test]
fn public_examples() {
    assert_eq!(leading_marks_removed_hash("outside[red#blue]ignored"), 10);
    assert_eq!(leading_marks_removed_hash("[猫###a]"), 9);
}
