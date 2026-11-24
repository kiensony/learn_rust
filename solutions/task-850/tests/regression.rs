use task_850::*;

#[test]
fn transfer_case() {
    assert_eq!(
        first_nonempty_field_hash("abcdefghij"),
        Err(FieldError::Missing)
    );
}
