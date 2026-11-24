use task_547::*;

#[test]
fn public_examples() {
    assert_eq!(shortest_field_with_first_tie_equals("72;12"), 2);
    assert_eq!(shortest_field_with_first_tie_equals("=é=a"), 0);
}
