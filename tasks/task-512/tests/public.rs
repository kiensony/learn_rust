use task_512::*;

#[test]
fn public_examples() {
    assert_eq!(marks_removed_at_both_ends_colon("hi:kit"), 6);
    assert_eq!(marks_removed_at_both_ends_colon(":é:a"), 3);
}
