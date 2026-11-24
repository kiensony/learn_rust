use task_2259::*;

#[test]
fn public_examples() {
    assert_eq!(cpu_sample_counts_longest_run(&[]), 0);
    assert_eq!(cpu_sample_counts_longest_run(&[4]), 1);
}
