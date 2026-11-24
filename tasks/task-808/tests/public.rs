use task_808::*;

#[test]
fn public_examples() {
    assert_eq!(remainder_after_first_mark_colon("hi:kit"), Ok(3));
    assert_eq!(
        remainder_after_first_mark_colon(":é:a"),
        Err(FieldError::TooLong)
    );
}
