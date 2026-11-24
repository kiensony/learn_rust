use task_906::*;

#[test]
fn public_examples() {
    assert_eq!(
        closing_marks_removed_colon("hi:kit", |s| s.chars().count() + 1),
        7
    );
    assert_eq!(closing_marks_removed_colon(":é:a", |s| s.len() * 2), 10);
}
