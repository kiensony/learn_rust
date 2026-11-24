use task_812::*;

#[test]
fn public_examples() {
    assert_eq!(shortest_field_with_first_tie_colon("hi:kit"), Ok(2));
    assert_eq!(
        shortest_field_with_first_tie_colon(":é:a"),
        Err(FieldError::Missing)
    );
}
