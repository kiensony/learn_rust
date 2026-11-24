use task_812::*;

#[test]
fn edge_cases() {
    assert_eq!(
        shortest_field_with_first_tie_colon(""),
        Err(FieldError::Missing)
    );
    assert_eq!(
        shortest_field_with_first_tie_colon(":"),
        Err(FieldError::Missing)
    );
    assert_eq!(shortest_field_with_first_tie_colon("cat"), Ok(3));
    assert_eq!(
        shortest_field_with_first_tie_colon("::one:猫:ab"),
        Err(FieldError::Missing)
    );
    assert_eq!(shortest_field_with_first_tie_colon("aa:bb"), Ok(2));
    assert_eq!(shortest_field_with_first_tie_colon("ok:🐈"), Ok(2));
    assert_eq!(
        shortest_field_with_first_tie_colon("abcdefghij"),
        Err(FieldError::TooLong)
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_812::TRACK_PROOF, 0x52540000 | 812);
}
