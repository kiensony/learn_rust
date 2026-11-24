use task_1254::*;

#[test]
fn public_examples() {
    assert_eq!(command_arguments_peaks(""), vec![]);
    assert_eq!(command_arguments_peaks("only"), vec![4u32]);
}
