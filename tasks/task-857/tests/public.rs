use task_857::*;

#[test]
fn public_examples() {
    assert_eq!(marks_removed_at_both_ends_colon("hi:kit"), 8);
    assert_eq!(marks_removed_at_both_ends_colon(":é:a"), 6);
}
