use task_569::*;

#[test]
fn public_examples() {
    assert_eq!(leading_marks_removed_slash("path/1/12"), "1/12");
    assert_eq!(leading_marks_removed_slash("path/123"), "123");
}
