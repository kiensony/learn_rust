use task_833::*;

#[test]
fn edge_cases() {
    assert_eq!(leading_field_equals("hi=kit"), Ok(2));
    assert_eq!(leading_field_equals("=é=a"), Err(FieldError::Missing));
    assert_eq!(leading_field_equals(""), Err(FieldError::Missing));
    assert_eq!(leading_field_equals("="), Err(FieldError::Missing));
    assert_eq!(leading_field_equals("cat"), Ok(3));
    assert_eq!(
        leading_field_equals("==one=猫=ab"),
        Err(FieldError::Missing)
    );
    assert_eq!(leading_field_equals("aa=bb"), Ok(2));
    assert_eq!(leading_field_equals("ok=🐈"), Ok(2));
    assert_eq!(leading_field_equals("aa=bb;meta=123"), Ok(2));
    assert_eq!(
        leading_field_equals(";ignore=all"),
        Err(FieldError::Missing)
    );
    assert_eq!(leading_field_equals("a==b;"), Ok(1));
    assert_eq!(leading_field_equals("abcdefghij"), Err(FieldError::TooLong));
}

#[test]
fn submission_marker() {
    assert_eq!(task_833::TRACK_PROOF, 0x52540000 | 833);
}
