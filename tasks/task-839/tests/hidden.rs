use task_839::*;

#[test]
fn edge_cases() {
    assert_eq!(prefix_before_final_mark_equals("hi=kit"), Ok(2));
    assert_eq!(prefix_before_final_mark_equals("=é=a"), Ok(3));
    assert_eq!(
        prefix_before_final_mark_equals(""),
        Err(FieldError::Missing)
    );
    assert_eq!(
        prefix_before_final_mark_equals("="),
        Err(FieldError::Missing)
    );
    assert_eq!(
        prefix_before_final_mark_equals("cat"),
        Err(FieldError::Missing)
    );
    assert_eq!(
        prefix_before_final_mark_equals("==one=猫=ab"),
        Err(FieldError::TooLong)
    );
    assert_eq!(prefix_before_final_mark_equals("aa=bb"), Ok(2));
    assert_eq!(prefix_before_final_mark_equals("ok=🐈"), Ok(2));
    assert_eq!(prefix_before_final_mark_equals("aa=bb;meta=123"), Ok(2));
    assert_eq!(
        prefix_before_final_mark_equals(";ignore=all"),
        Err(FieldError::Missing)
    );
    assert_eq!(prefix_before_final_mark_equals("a==b;"), Ok(2));
    assert_eq!(
        prefix_before_final_mark_equals("abcdefghij"),
        Err(FieldError::Missing)
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_839::TRACK_PROOF, 0x52540000 | 839);
}
