use task_841::*;

#[test]
fn edge_cases() {
    assert_eq!(longest_field_with_last_tie_equals("hi=kit"), Ok(3));
    assert_eq!(longest_field_with_last_tie_equals("=é=a"), Ok(2));
    assert_eq!(
        longest_field_with_last_tie_equals(""),
        Err(FieldError::Missing)
    );
    assert_eq!(
        longest_field_with_last_tie_equals("="),
        Err(FieldError::Missing)
    );
    assert_eq!(longest_field_with_last_tie_equals("cat"), Ok(3));
    assert_eq!(longest_field_with_last_tie_equals("==one=猫=ab"), Ok(3));
    assert_eq!(longest_field_with_last_tie_equals("aa=bb"), Ok(2));
    assert_eq!(longest_field_with_last_tie_equals("ok=🐈"), Ok(4));
    assert_eq!(longest_field_with_last_tie_equals("aa=bb;meta=123"), Ok(2));
    assert_eq!(
        longest_field_with_last_tie_equals(";ignore=all"),
        Err(FieldError::Missing)
    );
    assert_eq!(longest_field_with_last_tie_equals("a==b;"), Ok(1));
    assert_eq!(
        longest_field_with_last_tie_equals("abcdefghij"),
        Err(FieldError::TooLong)
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_841::TRACK_PROOF, 0x52540000 | 841);
}
