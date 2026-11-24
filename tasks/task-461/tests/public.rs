use task_461::*;

#[test]
fn public_examples() {
    assert_eq!(closing_marks_removed_colon(("hi:kit", "x")), 6);
    assert_eq!(closing_marks_removed_colon((":é:a", "z:w")), 5);
}
