use task_511::*;

#[test]
fn public_examples() {
    assert_eq!(closing_marks_removed_colon("hi:kit"), 6);
    assert_eq!(closing_marks_removed_colon(":é:a"), 4);
}
