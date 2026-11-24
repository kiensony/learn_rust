use task_520::*;

#[test]
fn public_examples() {
    assert_eq!(leading_marks_removed_slash("path/1/12"), 4);
    assert_eq!(leading_marks_removed_slash("path/123"), 3);
}
