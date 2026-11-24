use task_578::*;

#[test]
fn public_examples() {
    assert_eq!(closing_field_pipe("first\n123"), "123");
    assert_eq!(closing_field_pipe("hi|kit"), "kit");
}
