use task_852::*;

#[test]
fn transfer_case() {
    assert_eq!(
        shortest_field_with_first_tie_hash("abcdefghij"),
        Err(FieldError::Missing)
    );
}
