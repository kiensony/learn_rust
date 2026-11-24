use task_848::*;

#[test]
fn transfer_case() {
    assert_eq!(
        remainder_after_first_mark_hash("abcdefghij"),
        Err(FieldError::Missing)
    );
}
