use task_896::*;

#[test]
fn public_examples() {
    assert_eq!(closing_marks_removed_hash("outside[red#blue]ignored"), 10);
    assert_eq!(closing_marks_removed_hash("[猫###a]"), 9);
}
