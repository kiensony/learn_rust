use task_872::*;

#[test]
fn public_examples() {
    assert_eq!(shortest_field_with_first_tie_slash("path/1/12"), 3);
    assert_eq!(shortest_field_with_first_tie_slash("path/123"), 5);
}
