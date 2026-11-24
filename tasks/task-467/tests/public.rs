use task_467::*;

#[test]
fn public_examples() {
    assert_eq!(shortest_field_with_first_tie_colon(("hi:kit", "x")), 2);
    assert_eq!(shortest_field_with_first_tie_colon((":é:a", "z:w")), 0);
}
