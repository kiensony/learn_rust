use task_1078::*;

#[test]
fn public_examples() {
    assert_eq!(first_nonempty_field_pipe(&"first\n123"), 3);
    assert_eq!(first_nonempty_field_pipe(&"hi|kit"), 2);
}
