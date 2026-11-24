use task_1154::*;

#[test]
fn public_examples() {
    assert_eq!(closing_marks_removed_colon("hi:kit", "backup"), "hi:kit");
    assert_eq!(closing_marks_removed_colon(":é:a", "backup"), ":é:a");
}
