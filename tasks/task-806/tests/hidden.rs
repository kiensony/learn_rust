use task_806::*;

#[test]
fn edge_cases() {
    assert_eq!(
        closing_marks_removed_colon(":é:a"),
        Err(FieldError::TooLong)
    );
    assert_eq!(closing_marks_removed_colon(":"), Err(FieldError::Missing));
    assert_eq!(closing_marks_removed_colon("cat"), Ok(3));
    assert_eq!(
        closing_marks_removed_colon("::one:猫:ab"),
        Err(FieldError::TooLong)
    );
    assert_eq!(
        closing_marks_removed_colon("aa:bb"),
        Err(FieldError::TooLong)
    );
    assert_eq!(
        closing_marks_removed_colon("ok:🐈"),
        Err(FieldError::TooLong)
    );
    assert_eq!(
        closing_marks_removed_colon("abcdefghij"),
        Err(FieldError::TooLong)
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_806::TRACK_PROOF, 0x52540000 | 806);
}
