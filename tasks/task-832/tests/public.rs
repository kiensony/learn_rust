use task_832::*;

#[test]
fn public_examples() {
    assert_eq!(shortest_field_with_first_tie_pipe("first\n123"), Ok(3));
    assert_eq!(shortest_field_with_first_tie_pipe("hi|kit"), Ok(2));
}
