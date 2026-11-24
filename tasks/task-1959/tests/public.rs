use task_1959::*;

#[test]
fn public_examples() {
    assert_eq!(atomic_running_total_longest_run(&[]), 0);
    assert_eq!(atomic_running_total_longest_run(&[4]), 1);
}
