use task_826::*;

#[test]
fn edge_cases() {
    assert_eq!(
        closing_marks_removed_pipe("hi|kit"),
        Err(FieldError::TooLong)
    );
    assert_eq!(closing_marks_removed_pipe("|é|a"), Ok(5));
    assert_eq!(closing_marks_removed_pipe(""), Err(FieldError::Missing));
    assert_eq!(closing_marks_removed_pipe("|"), Err(FieldError::Missing));
    assert_eq!(closing_marks_removed_pipe("cat"), Ok(3));
    assert_eq!(
        closing_marks_removed_pipe("||one|猫|ab"),
        Err(FieldError::TooLong)
    );
    assert_eq!(closing_marks_removed_pipe("aa|bb"), Ok(5));
    assert_eq!(
        closing_marks_removed_pipe("ok|🐈"),
        Err(FieldError::TooLong)
    );
    assert_eq!(
        closing_marks_removed_pipe("header\nleft|right\r\n"),
        Err(FieldError::TooLong)
    );
    assert_eq!(
        closing_marks_removed_pipe("header\n\n"),
        Err(FieldError::Missing)
    );
    assert_eq!(closing_marks_removed_pipe("猫|x\nend|a"), Ok(5));
    assert_eq!(
        closing_marks_removed_pipe("abcdefghij"),
        Err(FieldError::TooLong)
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_826::TRACK_PROOF, 0x52540000 | 826);
}
