use task_2919::*;

#[test]
fn public_examples() {
    assert_eq!(latency_improvement_longest_run(&[]), 0);
    assert_eq!(latency_improvement_longest_run(&[4]), 0);
}
