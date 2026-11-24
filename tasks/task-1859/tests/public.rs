use task_1859::*;

#[test]
fn public_examples() {
    assert_eq!(shared_read_handles_longest_run(&[]), 0);
    assert_eq!(shared_read_handles_longest_run(&[4]), 1);
}
