use task_684::*;

#[test]
fn public_examples() {
    assert_eq!(
        shortest_field_with_first_tie_pipe(Event::Data("first\n123")),
        Some(3)
    );
    assert_eq!(
        shortest_field_with_first_tie_pipe(Event::Data("hi|kit")),
        Some(2)
    );
}
