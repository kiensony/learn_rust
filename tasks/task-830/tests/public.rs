use task_830::*;

#[test]
fn public_examples() {
    assert_eq!(first_nonempty_field_pipe("first\n123"), Ok(3));
    assert_eq!(first_nonempty_field_pipe("hi|kit"), Ok(2));
}
