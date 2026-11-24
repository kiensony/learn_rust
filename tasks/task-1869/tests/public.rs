use task_1869::*;

#[test]
fn public_examples() {
    assert_eq!(weak_read_handles_longest_run(&[]), 0);
    assert_eq!(weak_read_handles_longest_run(&[4]), 1);
}
