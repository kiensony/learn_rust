use task_716::*;

#[test]
fn public_examples() {
    assert_eq!(leading_marks_removed_slash(Some("path/1/12")), None);
    assert_eq!(leading_marks_removed_slash(Some("path/123")), Some(123));
}
