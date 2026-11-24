use task_683::*;

#[test]
fn public_examples() {
    assert_eq!(
        longest_field_with_last_tie_pipe(Event::Data("first\n123")),
        Some(3)
    );
    assert_eq!(
        longest_field_with_last_tie_pipe(Event::Data("|é|a")),
        Some(2)
    );
}
