use task_1251::*;

#[test]
fn public_examples() {
    assert_eq!(command_arguments_trace(""), vec![]);
    assert_eq!(command_arguments_trace("only"), vec![4u32]);
}
