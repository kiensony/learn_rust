use task_708::*;

#[test]
fn public_examples() {
    assert_eq!(closing_marks_removed_colon(Some("hi:kit")), None);
    assert_eq!(closing_marks_removed_colon(Some("3300")), Some(3300));
}
