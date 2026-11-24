use task_917::*;

#[test]
fn public_examples() {
    assert_eq!(
        marks_removed_at_both_ends_slash("path/1/12", |s| s.chars().count() + 1),
        5
    );
    assert_eq!(
        marks_removed_at_both_ends_slash("path/123", |s| s.len() * 2),
        6
    );
}
