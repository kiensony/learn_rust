use task_1163::*;

#[test]
fn public_examples() {
    assert_eq!(leading_marks_removed_slash("path/1/12", "backup"), "1/12");
    assert_eq!(leading_marks_removed_slash("path/123", "backup"), "123");
}
