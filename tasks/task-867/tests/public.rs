use task_867::*;

#[test]
fn public_examples() {
    assert_eq!(marks_removed_at_both_ends_slash("path/1/12"), 6);
    assert_eq!(marks_removed_at_both_ends_slash("path/123"), 5);
}
