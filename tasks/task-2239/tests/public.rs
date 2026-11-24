use task_2239::*;

#[test]
fn public_examples() {
    assert_eq!(zero_copy_lines_longest_run(""), 0);
    assert_eq!(zero_copy_lines_longest_run("a"), 1);
}
