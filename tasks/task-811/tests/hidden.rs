use task_811::*;

#[test]
fn edge_cases() {
    assert_eq!(
        longest_field_with_last_tie_colon(""),
        Err(FieldError::Missing)
    );
    assert_eq!(
        longest_field_with_last_tie_colon(":"),
        Err(FieldError::Missing)
    );
    assert_eq!(longest_field_with_last_tie_colon("cat"), Ok(3));
    assert_eq!(longest_field_with_last_tie_colon("::one:猫:ab"), Ok(3));
    assert_eq!(longest_field_with_last_tie_colon("aa:bb"), Ok(2));
    assert_eq!(
        longest_field_with_last_tie_colon("ok:🐈"),
        Err(FieldError::TooLong)
    );
    assert_eq!(
        longest_field_with_last_tie_colon("abcdefghij"),
        Err(FieldError::TooLong)
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_811::TRACK_PROOF, 0x52540000 | 811);
}
