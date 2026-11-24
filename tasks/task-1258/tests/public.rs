use task_1258::*;

#[test]
fn public_examples() {
    assert_eq!(command_arguments_steps(""), vec![]);
    assert_eq!(command_arguments_steps("only"), vec![]);
}
