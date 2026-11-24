use task_736::*;

#[test]
fn public_examples() {
    assert_eq!(leading_marks_removed_equals(Some("72;12")), Some(72));
    assert_eq!(leading_marks_removed_equals(Some("猫=12;note")), None);
}
