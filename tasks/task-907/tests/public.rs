use task_907::*;

#[test]
fn public_examples() {
    assert_eq!(
        marks_removed_at_both_ends_colon("hi:kit", |s| s.chars().count() + 1),
        7
    );
    assert_eq!(marks_removed_at_both_ends_colon(":é:a", |s| s.len() * 2), 8);
}
