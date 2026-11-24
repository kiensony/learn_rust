use task_1259::*;

#[test]
fn public_examples() {
    assert_eq!(command_arguments_longest_run(""), 0);
    assert_eq!(command_arguments_longest_run("only"), 1);
}
