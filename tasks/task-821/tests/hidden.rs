use task_821::*;

#[test]
fn edge_cases() {
    assert_eq!(longest_field_with_last_tie_slash("hi/kit"), Ok(3));
    assert_eq!(longest_field_with_last_tie_slash("/é/a"), Ok(2));
    assert_eq!(
        longest_field_with_last_tie_slash(""),
        Err(FieldError::Missing)
    );
    assert_eq!(
        longest_field_with_last_tie_slash("/"),
        Err(FieldError::Missing)
    );
    assert_eq!(longest_field_with_last_tie_slash("cat"), Ok(3));
    assert_eq!(longest_field_with_last_tie_slash("//one/猫/ab"), Ok(3));
    assert_eq!(longest_field_with_last_tie_slash("aa/bb"), Ok(2));
    assert_eq!(longest_field_with_last_tie_slash("ok/🐈"), Ok(4));
    assert_eq!(
        longest_field_with_last_tie_slash("path/first/last"),
        Err(FieldError::TooLong)
    );
    assert_eq!(longest_field_with_last_tie_slash("path/path/"), Ok(4));
    assert_eq!(
        longest_field_with_last_tie_slash("path/"),
        Err(FieldError::Missing)
    );
    assert_eq!(
        longest_field_with_last_tie_slash("abcdefghij"),
        Err(FieldError::TooLong)
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_821::TRACK_PROOF, 0x52540000 | 821);
}
