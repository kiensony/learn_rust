use task_718::*;

#[test]
fn public_examples() {
    assert_eq!(marks_removed_at_both_ends_slash(Some("path/1/12")), None);
    assert_eq!(
        marks_removed_at_both_ends_slash(Some("path/123")),
        Some(123)
    );
}
