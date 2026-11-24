use task_915::*;

#[test]
fn public_examples() {
    assert_eq!(
        leading_marks_removed_slash("path/1/12", |s| s.chars().count() + 1),
        5
    );
    assert_eq!(leading_marks_removed_slash("path/123", |s| s.len() * 2), 6);
}
