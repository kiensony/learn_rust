use task_843::*;

#[test]
fn public_examples() {
    assert_eq!(leading_field_hash("outside[red#blue]ignored"), Ok(3));
    assert_eq!(leading_field_hash("unclosed[cat"), Err(FieldError::Missing));
}
