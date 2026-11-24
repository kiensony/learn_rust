use task_868::*;

#[test]
fn public_examples() {
    assert_eq!(remainder_after_first_mark_slash("path/1/12"), 4);
    assert_eq!(remainder_after_first_mark_slash("path/123"), 2);
}
