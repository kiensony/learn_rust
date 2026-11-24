use task_1172::*;

#[test]
fn public_examples() {
    assert_eq!(closing_field_pipe("first\n123", "backup"), "123");
    assert_eq!(closing_field_pipe("hi|kit", "backup"), "kit");
}
