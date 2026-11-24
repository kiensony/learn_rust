use task_847::*;

#[test]
fn edge_cases() {
    assert_eq!(
        marks_removed_at_both_ends_hash("unclosed[cat"),
        Err(FieldError::Missing)
    );
    assert_eq!(
        marks_removed_at_both_ends_hash("[]"),
        Err(FieldError::Missing)
    );
    assert_eq!(marks_removed_at_both_ends_hash("[42]"), Ok(2));
    assert_eq!(marks_removed_at_both_ends_hash("[321]"), Ok(3));
    assert_eq!(
        marks_removed_at_both_ends_hash("hi#kit"),
        Err(FieldError::Missing)
    );
    assert_eq!(
        marks_removed_at_both_ends_hash("#é#a"),
        Err(FieldError::Missing)
    );
    assert_eq!(
        marks_removed_at_both_ends_hash(""),
        Err(FieldError::Missing)
    );
    assert_eq!(
        marks_removed_at_both_ends_hash("#"),
        Err(FieldError::Missing)
    );
    assert_eq!(
        marks_removed_at_both_ends_hash("cat"),
        Err(FieldError::Missing)
    );
    assert_eq!(
        marks_removed_at_both_ends_hash("##one#猫#ab"),
        Err(FieldError::Missing)
    );
    assert_eq!(
        marks_removed_at_both_ends_hash("aa#bb"),
        Err(FieldError::Missing)
    );
    assert_eq!(
        marks_removed_at_both_ends_hash("ok#🐈"),
        Err(FieldError::Missing)
    );
    assert_eq!(
        marks_removed_at_both_ends_hash("abcdefghij"),
        Err(FieldError::Missing)
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_847::TRACK_PROOF, 0x52540000 | 847);
}
