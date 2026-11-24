use task_2909::*;

#[test]
fn public_examples() {
    assert_eq!(latency_increases_longest_run(&[]), 0);
    assert_eq!(latency_increases_longest_run(&[4]), 0);
}
