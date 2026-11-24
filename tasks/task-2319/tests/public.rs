use task_2319::*;

#[test]
fn public_examples() {
    assert_eq!(paired_speedup_longest_run(&[]), 0);
    assert_eq!(paired_speedup_longest_run(&[4]), 0);
}
