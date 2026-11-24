use task_1156::*;

#[test]
fn public_examples() {
    assert_eq!(remainder_after_first_mark_colon("hi:kit", "backup"), "kit");
    assert_eq!(remainder_after_first_mark_colon(":é:a", "backup"), "é:a");
}
