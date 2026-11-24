use task_843::*;

#[test]
fn edge_cases() {
    assert_eq!(leading_field_hash("[猫###a]"), Ok(3));
    assert_eq!(leading_field_hash("[]"), Err(FieldError::Missing));
    assert_eq!(leading_field_hash("[42]"), Ok(2));
    assert_eq!(leading_field_hash("[321]"), Ok(3));
    assert_eq!(leading_field_hash("hi#kit"), Err(FieldError::Missing));
    assert_eq!(leading_field_hash("#é#a"), Err(FieldError::Missing));
    assert_eq!(leading_field_hash(""), Err(FieldError::Missing));
    assert_eq!(leading_field_hash("#"), Err(FieldError::Missing));
    assert_eq!(leading_field_hash("cat"), Err(FieldError::Missing));
    assert_eq!(leading_field_hash("##one#猫#ab"), Err(FieldError::Missing));
    assert_eq!(leading_field_hash("aa#bb"), Err(FieldError::Missing));
    assert_eq!(leading_field_hash("ok#🐈"), Err(FieldError::Missing));
    assert_eq!(leading_field_hash("abcdefghij"), Err(FieldError::Missing));
}

#[test]
fn submission_marker() {
    assert_eq!(task_843::TRACK_PROOF, 0x52540000 | 843);
}
