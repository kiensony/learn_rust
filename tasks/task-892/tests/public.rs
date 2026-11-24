use task_892::*;

#[test]
fn public_examples() {
    assert_eq!(shortest_field_with_first_tie_equals("72;12"), 4);
    assert_eq!(shortest_field_with_first_tie_equals("=é=a"), 2);
}
