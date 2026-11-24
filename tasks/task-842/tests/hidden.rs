use task_842::*;

#[test]
fn edge_cases() {
    assert_eq!(shortest_field_with_first_tie_equals("猫=12;note"), Ok(2));
    assert_eq!(shortest_field_with_first_tie_equals("hi=kit"), Ok(2));
    assert_eq!(
        shortest_field_with_first_tie_equals(""),
        Err(FieldError::Missing)
    );
    assert_eq!(
        shortest_field_with_first_tie_equals("="),
        Err(FieldError::Missing)
    );
    assert_eq!(shortest_field_with_first_tie_equals("cat"), Ok(3));
    assert_eq!(
        shortest_field_with_first_tie_equals("==one=猫=ab"),
        Err(FieldError::Missing)
    );
    assert_eq!(shortest_field_with_first_tie_equals("aa=bb"), Ok(2));
    assert_eq!(shortest_field_with_first_tie_equals("ok=🐈"), Ok(2));
    assert_eq!(
        shortest_field_with_first_tie_equals("aa=bb;meta=123"),
        Ok(2)
    );
    assert_eq!(
        shortest_field_with_first_tie_equals(";ignore=all"),
        Err(FieldError::Missing)
    );
    assert_eq!(
        shortest_field_with_first_tie_equals("a==b;"),
        Err(FieldError::Missing)
    );
    assert_eq!(
        shortest_field_with_first_tie_equals("abcdefghij"),
        Err(FieldError::TooLong)
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_842::TRACK_PROOF, 0x52540000 | 842);
}
