use task_838::*;

#[test]
fn edge_cases() {
    assert_eq!(remainder_after_first_mark_equals("hi=kit"), Ok(3));
    assert_eq!(remainder_after_first_mark_equals("=é=a"), Ok(4));
    assert_eq!(
        remainder_after_first_mark_equals(""),
        Err(FieldError::Missing)
    );
    assert_eq!(
        remainder_after_first_mark_equals("="),
        Err(FieldError::Missing)
    );
    assert_eq!(
        remainder_after_first_mark_equals("cat"),
        Err(FieldError::Missing)
    );
    assert_eq!(
        remainder_after_first_mark_equals("==one=猫=ab"),
        Err(FieldError::TooLong)
    );
    assert_eq!(remainder_after_first_mark_equals("aa=bb"), Ok(2));
    assert_eq!(remainder_after_first_mark_equals("ok=🐈"), Ok(4));
    assert_eq!(remainder_after_first_mark_equals("aa=bb;meta=123"), Ok(2));
    assert_eq!(
        remainder_after_first_mark_equals(";ignore=all"),
        Err(FieldError::Missing)
    );
    assert_eq!(remainder_after_first_mark_equals("a==b;"), Ok(2));
    assert_eq!(
        remainder_after_first_mark_equals("abcdefghij"),
        Err(FieldError::Missing)
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_838::TRACK_PROOF, 0x52540000 | 838);
}
