use task_562::*;

#[test]
fn public_examples() {
    assert_eq!(marks_removed_at_both_ends_colon("hi:kit"), "hi:kit");
    assert_eq!(marks_removed_at_both_ends_colon(":é:a"), "é:a");
}
