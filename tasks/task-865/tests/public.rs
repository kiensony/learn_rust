use task_865::*;

#[test]
fn public_examples() {
    assert_eq!(leading_marks_removed_slash("path/1/12"), 6);
    assert_eq!(leading_marks_removed_slash("path/123"), 5);
}
