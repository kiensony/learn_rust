use task_817::*;

#[test]
fn public_examples() {
    assert_eq!(marks_removed_at_both_ends_slash("path/1/12"), Ok(4));
    assert_eq!(marks_removed_at_both_ends_slash("path/123"), Ok(3));
}
