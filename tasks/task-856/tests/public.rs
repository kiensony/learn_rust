use task_856::*;

#[test]
fn public_examples() {
    assert_eq!(closing_marks_removed_colon("hi:kit"), 8);
    assert_eq!(closing_marks_removed_colon(":é:a"), 7);
}
