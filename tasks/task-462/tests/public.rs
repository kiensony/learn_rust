use task_462::*;

#[test]
fn public_examples() {
    assert_eq!(marks_removed_at_both_ends_colon(("hi:kit", "x")), 6);
    assert_eq!(marks_removed_at_both_ends_colon((":é:a", "z:w")), 4);
}
