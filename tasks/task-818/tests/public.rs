use task_818::*;

#[test]
fn public_examples() {
    assert_eq!(remainder_after_first_mark_slash("path/1/12"), Ok(2));
    assert_eq!(
        remainder_after_first_mark_slash("path/123"),
        Err(FieldError::Missing)
    );
}
