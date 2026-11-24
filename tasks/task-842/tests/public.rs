use task_842::*;

#[test]
fn public_examples() {
    assert_eq!(shortest_field_with_first_tie_equals("72;12"), Ok(2));
    assert_eq!(
        shortest_field_with_first_tie_equals("=é=a"),
        Err(FieldError::Missing)
    );
}
