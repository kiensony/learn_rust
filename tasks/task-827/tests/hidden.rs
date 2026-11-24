use task_827::*;

#[test]
fn edge_cases() {
    assert_eq!(
        marks_removed_at_both_ends_pipe("hi|kit"),
        Err(FieldError::TooLong)
    );
    assert_eq!(marks_removed_at_both_ends_pipe("|é|a"), Ok(4));
    assert_eq!(
        marks_removed_at_both_ends_pipe(""),
        Err(FieldError::Missing)
    );
    assert_eq!(
        marks_removed_at_both_ends_pipe("|"),
        Err(FieldError::Missing)
    );
    assert_eq!(marks_removed_at_both_ends_pipe("cat"), Ok(3));
    assert_eq!(
        marks_removed_at_both_ends_pipe("||one|猫|ab"),
        Err(FieldError::TooLong)
    );
    assert_eq!(marks_removed_at_both_ends_pipe("aa|bb"), Ok(5));
    assert_eq!(
        marks_removed_at_both_ends_pipe("ok|🐈"),
        Err(FieldError::TooLong)
    );
    assert_eq!(
        marks_removed_at_both_ends_pipe("header\nleft|right\r\n"),
        Err(FieldError::TooLong)
    );
    assert_eq!(
        marks_removed_at_both_ends_pipe("header\n\n"),
        Err(FieldError::Missing)
    );
    assert_eq!(marks_removed_at_both_ends_pipe("猫|x\nend|a"), Ok(5));
    assert_eq!(
        marks_removed_at_both_ends_pipe("abcdefghij"),
        Err(FieldError::TooLong)
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_827::TRACK_PROOF, 0x52540000 | 827);
}
