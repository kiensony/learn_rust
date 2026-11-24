use task_837::*;

#[test]
fn edge_cases() {
    assert_eq!(marks_removed_at_both_ends_equals("hi=kit"), Ok(6));
    assert_eq!(marks_removed_at_both_ends_equals("=é=a"), Ok(4));
    assert_eq!(
        marks_removed_at_both_ends_equals(""),
        Err(FieldError::Missing)
    );
    assert_eq!(
        marks_removed_at_both_ends_equals("="),
        Err(FieldError::Missing)
    );
    assert_eq!(marks_removed_at_both_ends_equals("cat"), Ok(3));
    assert_eq!(
        marks_removed_at_both_ends_equals("==one=猫=ab"),
        Err(FieldError::TooLong)
    );
    assert_eq!(marks_removed_at_both_ends_equals("aa=bb"), Ok(5));
    assert_eq!(
        marks_removed_at_both_ends_equals("ok=🐈"),
        Err(FieldError::TooLong)
    );
    assert_eq!(marks_removed_at_both_ends_equals("aa=bb;meta=123"), Ok(5));
    assert_eq!(
        marks_removed_at_both_ends_equals(";ignore=all"),
        Err(FieldError::Missing)
    );
    assert_eq!(marks_removed_at_both_ends_equals("a==b;"), Ok(4));
    assert_eq!(
        marks_removed_at_both_ends_equals("abcdefghij"),
        Err(FieldError::TooLong)
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_837::TRACK_PROOF, 0x52540000 | 837);
}
