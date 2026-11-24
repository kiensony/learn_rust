use task_463::*;

#[test]
fn public_examples() {
    assert_eq!(remainder_after_first_mark_colon(("hi:kit", "x")), 3);
    assert_eq!(remainder_after_first_mark_colon((":é:a", "z:w")), 4);
}
