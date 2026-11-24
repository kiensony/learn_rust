use task_810::*;

#[test]
fn public_examples() {
    assert_eq!(first_nonempty_field_colon("hi:kit"), Ok(2));
    assert_eq!(first_nonempty_field_colon(""), Err(FieldError::Missing));
}
