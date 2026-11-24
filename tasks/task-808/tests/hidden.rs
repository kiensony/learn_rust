use task_808::*;

#[test]
fn edge_cases() {
    assert_eq!(
        remainder_after_first_mark_colon(""),
        Err(FieldError::Missing)
    );
    assert_eq!(
        remainder_after_first_mark_colon(":"),
        Err(FieldError::Missing)
    );
    assert_eq!(
        remainder_after_first_mark_colon("cat"),
        Err(FieldError::Missing)
    );
    assert_eq!(
        remainder_after_first_mark_colon("::one:猫:ab"),
        Err(FieldError::TooLong)
    );
    assert_eq!(remainder_after_first_mark_colon("aa:bb"), Ok(2));
    assert_eq!(
        remainder_after_first_mark_colon("ok:🐈"),
        Err(FieldError::TooLong)
    );
    assert_eq!(
        remainder_after_first_mark_colon("abcdefghij"),
        Err(FieldError::Missing)
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_808::TRACK_PROOF, 0x52540000 | 808);
}
