use task_807::*;

#[test]
fn edge_cases() {
    assert_eq!(
        marks_removed_at_both_ends_colon(":é:a"),
        Err(FieldError::TooLong)
    );
    assert_eq!(
        marks_removed_at_both_ends_colon(":"),
        Err(FieldError::Missing)
    );
    assert_eq!(marks_removed_at_both_ends_colon("cat"), Ok(3));
    assert_eq!(
        marks_removed_at_both_ends_colon("::one:猫:ab"),
        Err(FieldError::TooLong)
    );
    assert_eq!(
        marks_removed_at_both_ends_colon("aa:bb"),
        Err(FieldError::TooLong)
    );
    assert_eq!(
        marks_removed_at_both_ends_colon("ok:🐈"),
        Err(FieldError::TooLong)
    );
    assert_eq!(
        marks_removed_at_both_ends_colon("abcdefghij"),
        Err(FieldError::TooLong)
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_807::TRACK_PROOF, 0x52540000 | 807);
}
