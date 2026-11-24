use task_828::*;

#[test]
fn edge_cases() {
    assert_eq!(remainder_after_first_mark_pipe("hi|kit"), Ok(3));
    assert_eq!(remainder_after_first_mark_pipe("|é|a"), Ok(4));
    assert_eq!(
        remainder_after_first_mark_pipe(""),
        Err(FieldError::Missing)
    );
    assert_eq!(
        remainder_after_first_mark_pipe("|"),
        Err(FieldError::Missing)
    );
    assert_eq!(
        remainder_after_first_mark_pipe("cat"),
        Err(FieldError::Missing)
    );
    assert_eq!(
        remainder_after_first_mark_pipe("||one|猫|ab"),
        Err(FieldError::TooLong)
    );
    assert_eq!(remainder_after_first_mark_pipe("aa|bb"), Ok(2));
    assert_eq!(remainder_after_first_mark_pipe("ok|🐈"), Ok(4));
    assert_eq!(
        remainder_after_first_mark_pipe("header\nleft|right\r\n"),
        Ok(5)
    );
    assert_eq!(
        remainder_after_first_mark_pipe("header\n\n"),
        Err(FieldError::Missing)
    );
    assert_eq!(remainder_after_first_mark_pipe("猫|x\nend|a"), Ok(1));
    assert_eq!(
        remainder_after_first_mark_pipe("abcdefghij"),
        Err(FieldError::Missing)
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_828::TRACK_PROOF, 0x52540000 | 828);
}
