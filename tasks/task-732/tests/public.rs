use task_732::*;

#[test]
fn public_examples() {
    assert_eq!(
        longest_field_with_last_tie_pipe(Some("first\n123")),
        Some(123)
    );
    assert_eq!(longest_field_with_last_tie_pipe(Some("hi|kit")), None);
}
