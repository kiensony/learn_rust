use task_1257::*;

#[test]
fn public_examples() {
    assert_eq!(command_arguments_distinct(""), vec![]);
    assert_eq!(command_arguments_distinct("only"), vec![4u32]);
}
