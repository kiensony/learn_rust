use task_2929::*;

#[test]
fn public_examples() {
    assert_eq!(failure_threshold_longest_run(&[]), 0);
    assert_eq!(failure_threshold_longest_run(&[4]), 0);
}
