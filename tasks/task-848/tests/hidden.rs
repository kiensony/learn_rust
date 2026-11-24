use task_848::*;

#[test]
fn edge_cases() {
    assert_eq!(
        remainder_after_first_mark_hash("unclosed[cat"),
        Err(FieldError::Missing)
    );
    assert_eq!(
        remainder_after_first_mark_hash("[]"),
        Err(FieldError::Missing)
    );
    assert_eq!(
        remainder_after_first_mark_hash("[42]"),
        Err(FieldError::Missing)
    );
    assert_eq!(
        remainder_after_first_mark_hash("[321]"),
        Err(FieldError::Missing)
    );
    assert_eq!(
        remainder_after_first_mark_hash("hi#kit"),
        Err(FieldError::Missing)
    );
    assert_eq!(
        remainder_after_first_mark_hash("#é#a"),
        Err(FieldError::Missing)
    );
    assert_eq!(
        remainder_after_first_mark_hash(""),
        Err(FieldError::Missing)
    );
    assert_eq!(
        remainder_after_first_mark_hash("#"),
        Err(FieldError::Missing)
    );
    assert_eq!(
        remainder_after_first_mark_hash("cat"),
        Err(FieldError::Missing)
    );
    assert_eq!(
        remainder_after_first_mark_hash("##one#猫#ab"),
        Err(FieldError::Missing)
    );
    assert_eq!(
        remainder_after_first_mark_hash("aa#bb"),
        Err(FieldError::Missing)
    );
    assert_eq!(
        remainder_after_first_mark_hash("ok#🐈"),
        Err(FieldError::Missing)
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_848::TRACK_PROOF, 0x52540000 | 848);
}
