use task_517::*;

#[test]
fn public_examples() {
    assert_eq!(shortest_field_with_first_tie_colon("hi:kit"), 2);
    assert_eq!(shortest_field_with_first_tie_colon(":é:a"), 0);
}
