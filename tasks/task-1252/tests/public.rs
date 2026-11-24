use task_1252::*;

#[test]
fn public_examples() {
    assert_eq!(command_arguments_total(""), 0);
    assert_eq!(command_arguments_total("only"), 4);
}
