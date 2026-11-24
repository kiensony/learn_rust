use task_850::*;

#[test]
fn public_examples() {
    assert_eq!(first_nonempty_field_hash("outside[red#blue]ignored"), Ok(3));
    assert_eq!(
        first_nonempty_field_hash("unclosed[cat"),
        Err(FieldError::Missing)
    );
}
