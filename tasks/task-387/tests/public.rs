use task_387::*;

#[test]
fn public_examples() {
    assert_eq!(
        shortest_field_with_first_tie_pipe(&String::from("first\n123")),
        3
    );
    assert_eq!(
        shortest_field_with_first_tie_pipe(&String::from("hi|kit")),
        2
    );
}
