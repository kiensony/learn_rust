use task_873::*;

#[test]
fn public_examples() {
    assert_eq!(leading_field_pipe("first\n123"), 5);
    assert_eq!(leading_field_pipe("hi|kit"), 4);
}
