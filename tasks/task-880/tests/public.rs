use task_880::*;

#[test]
fn public_examples() {
    assert_eq!(first_nonempty_field_pipe("first\n123"), 5);
    assert_eq!(first_nonempty_field_pipe("hi|kit"), 4);
}
