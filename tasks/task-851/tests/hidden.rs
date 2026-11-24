use task_851::*;

#[test]
fn edge_cases() {
    assert_eq!(
        longest_field_with_last_tie_hash("unclosed[cat"),
        Err(FieldError::Missing)
    );
    assert_eq!(
        longest_field_with_last_tie_hash("[]"),
        Err(FieldError::Missing)
    );
    assert_eq!(longest_field_with_last_tie_hash("[42]"), Ok(2));
    assert_eq!(longest_field_with_last_tie_hash("[321]"), Ok(3));
    assert_eq!(
        longest_field_with_last_tie_hash("hi#kit"),
        Err(FieldError::Missing)
    );
    assert_eq!(
        longest_field_with_last_tie_hash("#é#a"),
        Err(FieldError::Missing)
    );
    assert_eq!(
        longest_field_with_last_tie_hash(""),
        Err(FieldError::Missing)
    );
    assert_eq!(
        longest_field_with_last_tie_hash("#"),
        Err(FieldError::Missing)
    );
    assert_eq!(
        longest_field_with_last_tie_hash("cat"),
        Err(FieldError::Missing)
    );
    assert_eq!(
        longest_field_with_last_tie_hash("##one#猫#ab"),
        Err(FieldError::Missing)
    );
    assert_eq!(
        longest_field_with_last_tie_hash("aa#bb"),
        Err(FieldError::Missing)
    );
    assert_eq!(
        longest_field_with_last_tie_hash("ok#🐈"),
        Err(FieldError::Missing)
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_851::TRACK_PROOF, 0x52540000 | 851);
}
