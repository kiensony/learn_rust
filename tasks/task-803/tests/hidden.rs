use task_803::*;

#[test]
fn edge_cases() {
    assert_eq!(leading_field_colon(""), Err(FieldError::Missing));
    assert_eq!(leading_field_colon(":"), Err(FieldError::Missing));
    assert_eq!(leading_field_colon("cat"), Ok(3));
    assert_eq!(leading_field_colon("::one:猫:ab"), Err(FieldError::Missing));
    assert_eq!(leading_field_colon("aa:bb"), Ok(2));
    assert_eq!(leading_field_colon("ok:🐈"), Ok(2));
    assert_eq!(leading_field_colon("abcdefghij"), Err(FieldError::TooLong));
}

#[test]
fn submission_marker() {
    assert_eq!(task_803::TRACK_PROOF, 0x52540000 | 803);
}
