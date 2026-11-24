use task_815::*;

#[test]
fn public_examples() {
    assert_eq!(leading_marks_removed_slash("path/1/12"), Ok(4));
    assert_eq!(leading_marks_removed_slash("path/123"), Ok(3));
}
