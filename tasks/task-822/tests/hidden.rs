use task_822::*;

#[test]
fn edge_cases() {
    assert_eq!(shortest_field_with_first_tie_slash("hi/kit"), Ok(2));
    assert_eq!(
        shortest_field_with_first_tie_slash("/é/a"),
        Err(FieldError::Missing)
    );
    assert_eq!(
        shortest_field_with_first_tie_slash(""),
        Err(FieldError::Missing)
    );
    assert_eq!(
        shortest_field_with_first_tie_slash("/"),
        Err(FieldError::Missing)
    );
    assert_eq!(shortest_field_with_first_tie_slash("cat"), Ok(3));
    assert_eq!(
        shortest_field_with_first_tie_slash("//one/猫/ab"),
        Err(FieldError::Missing)
    );
    assert_eq!(shortest_field_with_first_tie_slash("aa/bb"), Ok(2));
    assert_eq!(shortest_field_with_first_tie_slash("ok/🐈"), Ok(2));
    assert_eq!(
        shortest_field_with_first_tie_slash("path/first/last"),
        Ok(4)
    );
    assert_eq!(
        shortest_field_with_first_tie_slash("path/path/"),
        Err(FieldError::Missing)
    );
    assert_eq!(
        shortest_field_with_first_tie_slash("path/"),
        Err(FieldError::Missing)
    );
    assert_eq!(
        shortest_field_with_first_tie_slash("abcdefghij"),
        Err(FieldError::TooLong)
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_822::TRACK_PROOF, 0x52540000 | 822);
}
