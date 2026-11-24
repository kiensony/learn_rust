use task_858::*;

#[test]
fn public_examples() {
    assert_eq!(remainder_after_first_mark_colon("hi:kit"), 5);
    assert_eq!(remainder_after_first_mark_colon(":é:a"), 6);
}
