use task_1255::*;

#[test]
fn public_examples() {
    assert_eq!(command_arguments_changes(""), vec![]);
    assert_eq!(command_arguments_changes("only"), vec![4u32]);
}
