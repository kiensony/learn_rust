use task_844::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_field_hash("unclosed[cat"), Err(FieldError::Missing));
    assert_eq!(closing_field_hash("[]"), Err(FieldError::Missing));
    assert_eq!(closing_field_hash("[42]"), Ok(2));
    assert_eq!(closing_field_hash("[321]"), Ok(3));
    assert_eq!(closing_field_hash("hi#kit"), Err(FieldError::Missing));
    assert_eq!(closing_field_hash("#é#a"), Err(FieldError::Missing));
    assert_eq!(closing_field_hash(""), Err(FieldError::Missing));
    assert_eq!(closing_field_hash("#"), Err(FieldError::Missing));
    assert_eq!(closing_field_hash("cat"), Err(FieldError::Missing));
    assert_eq!(closing_field_hash("##one#猫#ab"), Err(FieldError::Missing));
    assert_eq!(closing_field_hash("aa#bb"), Err(FieldError::Missing));
    assert_eq!(closing_field_hash("ok#🐈"), Err(FieldError::Missing));
    assert_eq!(closing_field_hash("abcdefghij"), Err(FieldError::Missing));
}

#[test]
fn submission_marker() {
    assert_eq!(task_844::TRACK_PROOF, 0x52540000 | 844);
}
