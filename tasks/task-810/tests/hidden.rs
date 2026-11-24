use task_810::*;

#[test]
fn edge_cases() {
    assert_eq!(first_nonempty_field_colon(":é:a"), Ok(2));
    assert_eq!(first_nonempty_field_colon(":"), Err(FieldError::Missing));
    assert_eq!(first_nonempty_field_colon("cat"), Ok(3));
    assert_eq!(first_nonempty_field_colon("::one:猫:ab"), Ok(3));
    assert_eq!(first_nonempty_field_colon("aa:bb"), Ok(2));
    assert_eq!(first_nonempty_field_colon("ok:🐈"), Ok(2));
    assert_eq!(
        first_nonempty_field_colon("abcdefghij"),
        Err(FieldError::TooLong)
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_810::TRACK_PROOF, 0x52540000 | 810);
}
