use task_572::*;

#[test]
fn public_examples() {
    assert_eq!(remainder_after_first_mark_slash("path/1/12"), "12");
    assert_eq!(remainder_after_first_mark_slash("path/123"), "");
}
