use task_846::*;

#[test]
fn edge_cases() {
    assert_eq!(
        closing_marks_removed_hash("unclosed[cat"),
        Err(FieldError::Missing)
    );
    assert_eq!(closing_marks_removed_hash("[]"), Err(FieldError::Missing));
    assert_eq!(closing_marks_removed_hash("[42]"), Ok(2));
    assert_eq!(closing_marks_removed_hash("[321]"), Ok(3));
    assert_eq!(
        closing_marks_removed_hash("hi#kit"),
        Err(FieldError::Missing)
    );
    assert_eq!(closing_marks_removed_hash("#é#a"), Err(FieldError::Missing));
    assert_eq!(closing_marks_removed_hash(""), Err(FieldError::Missing));
    assert_eq!(closing_marks_removed_hash("#"), Err(FieldError::Missing));
    assert_eq!(closing_marks_removed_hash("cat"), Err(FieldError::Missing));
    assert_eq!(
        closing_marks_removed_hash("##one#猫#ab"),
        Err(FieldError::Missing)
    );
    assert_eq!(
        closing_marks_removed_hash("aa#bb"),
        Err(FieldError::Missing)
    );
    assert_eq!(
        closing_marks_removed_hash("ok#🐈"),
        Err(FieldError::Missing)
    );
    assert_eq!(
        closing_marks_removed_hash("abcdefghij"),
        Err(FieldError::Missing)
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_846::TRACK_PROOF, 0x52540000 | 846);
}
