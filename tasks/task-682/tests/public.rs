use task_682::*;

#[test]
fn public_examples() {
    assert_eq!(
        first_nonempty_field_pipe(Event::Data("first\n123")),
        Some(3)
    );
    assert_eq!(first_nonempty_field_pipe(Event::Data("hi|kit")), Some(2));
}
