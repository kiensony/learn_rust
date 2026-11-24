use task_832::*;

#[test]
fn edge_cases() {
    assert_eq!(shortest_field_with_first_tie_pipe("header\n猫|123"), Ok(3));
    assert_eq!(
        shortest_field_with_first_tie_pipe("|é|a"),
        Err(FieldError::Missing)
    );
    assert_eq!(
        shortest_field_with_first_tie_pipe(""),
        Err(FieldError::Missing)
    );
    assert_eq!(
        shortest_field_with_first_tie_pipe("|"),
        Err(FieldError::Missing)
    );
    assert_eq!(shortest_field_with_first_tie_pipe("cat"), Ok(3));
    assert_eq!(
        shortest_field_with_first_tie_pipe("||one|猫|ab"),
        Err(FieldError::Missing)
    );
    assert_eq!(shortest_field_with_first_tie_pipe("aa|bb"), Ok(2));
    assert_eq!(shortest_field_with_first_tie_pipe("ok|🐈"), Ok(2));
    assert_eq!(
        shortest_field_with_first_tie_pipe("header\nleft|right\r\n"),
        Ok(4)
    );
    assert_eq!(
        shortest_field_with_first_tie_pipe("header\n\n"),
        Err(FieldError::Missing)
    );
    assert_eq!(shortest_field_with_first_tie_pipe("猫|x\nend|a"), Ok(1));
    assert_eq!(
        shortest_field_with_first_tie_pipe("abcdefghij"),
        Err(FieldError::TooLong)
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_832::TRACK_PROOF, 0x52540000 | 832);
}
