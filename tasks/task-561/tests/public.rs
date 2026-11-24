use task_561::*;

#[test]
fn public_examples() {
    assert_eq!(closing_marks_removed_colon("hi:kit"), "hi:kit");
    assert_eq!(closing_marks_removed_colon(":é:a"), ":é:a");
}
