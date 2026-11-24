use task_722::*;

#[test]
fn public_examples() {
    assert_eq!(
        longest_field_with_last_tie_slash(Some("path/1/12")),
        Some(12)
    );
    assert_eq!(
        longest_field_with_last_tie_slash(Some("path/123")),
        Some(123)
    );
}
