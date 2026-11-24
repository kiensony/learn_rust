use task_1121::*;

#[test]
fn public_examples() {
    assert_eq!(leading_field_pipe(&"first\n123"), 3);
    assert_eq!(leading_field_pipe(&"hi|kit"), 2);
}
