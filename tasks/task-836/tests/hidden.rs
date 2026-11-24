use task_836::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_marks_removed_equals("hi=kit"), Ok(6));
    assert_eq!(closing_marks_removed_equals("=é=a"), Ok(5));
    assert_eq!(closing_marks_removed_equals(""), Err(FieldError::Missing));
    assert_eq!(closing_marks_removed_equals("="), Err(FieldError::Missing));
    assert_eq!(closing_marks_removed_equals("cat"), Ok(3));
    assert_eq!(
        closing_marks_removed_equals("==one=猫=ab"),
        Err(FieldError::TooLong)
    );
    assert_eq!(closing_marks_removed_equals("aa=bb"), Ok(5));
    assert_eq!(
        closing_marks_removed_equals("ok=🐈"),
        Err(FieldError::TooLong)
    );
    assert_eq!(closing_marks_removed_equals("aa=bb;meta=123"), Ok(5));
    assert_eq!(
        closing_marks_removed_equals(";ignore=all"),
        Err(FieldError::Missing)
    );
    assert_eq!(closing_marks_removed_equals("a==b;"), Ok(4));
    assert_eq!(
        closing_marks_removed_equals("abcdefghij"),
        Err(FieldError::TooLong)
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_836::TRACK_PROOF, 0x52540000 | 836);
}
