use task_803::*;

#[test]
fn public_examples() {
    assert_eq!(leading_field_colon("hi:kit"), Ok(2));
    assert_eq!(leading_field_colon(":é:a"), Err(FieldError::Missing));
}
