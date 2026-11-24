use task_823::*;

#[test]
fn edge_cases() {
    assert_eq!(leading_field_pipe("header\n猫|123"), Ok(3));
    assert_eq!(leading_field_pipe("|é|a"), Err(FieldError::Missing));
    assert_eq!(leading_field_pipe(""), Err(FieldError::Missing));
    assert_eq!(leading_field_pipe("|"), Err(FieldError::Missing));
    assert_eq!(leading_field_pipe("cat"), Ok(3));
    assert_eq!(leading_field_pipe("||one|猫|ab"), Err(FieldError::Missing));
    assert_eq!(leading_field_pipe("aa|bb"), Ok(2));
    assert_eq!(leading_field_pipe("ok|🐈"), Ok(2));
    assert_eq!(leading_field_pipe("header\nleft|right\r\n"), Ok(4));
    assert_eq!(leading_field_pipe("header\n\n"), Err(FieldError::Missing));
    assert_eq!(leading_field_pipe("猫|x\nend|a"), Ok(3));
    assert_eq!(leading_field_pipe("abcdefghij"), Err(FieldError::TooLong));
}

#[test]
fn submission_marker() {
    assert_eq!(task_823::TRACK_PROOF, 0x52540000 | 823);
}
