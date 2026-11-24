use task_1253::*;

#[test]
fn public_examples() {
    assert_eq!(command_arguments_bounds(""), None);
    assert_eq!(command_arguments_bounds("only"), Some((4u32, 4u32)));
}
