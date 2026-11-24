use task_804::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_field_colon(""), Err(FieldError::Missing));
    assert_eq!(closing_field_colon(":"), Err(FieldError::Missing));
    assert_eq!(closing_field_colon("cat"), Ok(3));
    assert_eq!(closing_field_colon("::one:猫:ab"), Ok(2));
    assert_eq!(closing_field_colon("aa:bb"), Ok(2));
    assert_eq!(closing_field_colon("ok:🐈"), Err(FieldError::TooLong));
    assert_eq!(closing_field_colon("abcdefghij"), Err(FieldError::TooLong));
}

#[test]
fn submission_marker() {
    assert_eq!(task_804::TRACK_PROOF, 0x52540000 | 804);
}
