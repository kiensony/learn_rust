use task_831::*;

#[test]
fn edge_cases() {
    assert_eq!(longest_field_with_last_tie_pipe("header\n猫|123"), Ok(3));
    assert_eq!(longest_field_with_last_tie_pipe("hi|kit"), Ok(3));
    assert_eq!(
        longest_field_with_last_tie_pipe(""),
        Err(FieldError::Missing)
    );
    assert_eq!(
        longest_field_with_last_tie_pipe("|"),
        Err(FieldError::Missing)
    );
    assert_eq!(longest_field_with_last_tie_pipe("cat"), Ok(3));
    assert_eq!(longest_field_with_last_tie_pipe("||one|猫|ab"), Ok(3));
    assert_eq!(longest_field_with_last_tie_pipe("aa|bb"), Ok(2));
    assert_eq!(longest_field_with_last_tie_pipe("ok|🐈"), Ok(4));
    assert_eq!(
        longest_field_with_last_tie_pipe("header\nleft|right\r\n"),
        Ok(5)
    );
    assert_eq!(
        longest_field_with_last_tie_pipe("header\n\n"),
        Err(FieldError::Missing)
    );
    assert_eq!(longest_field_with_last_tie_pipe("猫|x\nend|a"), Ok(3));
    assert_eq!(
        longest_field_with_last_tie_pipe("abcdefghij"),
        Err(FieldError::TooLong)
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_831::TRACK_PROOF, 0x52540000 | 831);
}
