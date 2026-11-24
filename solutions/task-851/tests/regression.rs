use task_851::*;

#[test]
fn transfer_case() {
    assert_eq!(
        longest_field_with_last_tie_hash("abcdefghij"),
        Err(FieldError::Missing)
    );
}
