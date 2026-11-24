use task_822::*;

#[test]
fn public_examples() {
    assert_eq!(shortest_field_with_first_tie_slash("path/1/12"), Ok(1));
    assert_eq!(shortest_field_with_first_tie_slash("path/123"), Ok(3));
}
