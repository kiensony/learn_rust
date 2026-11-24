use task_2252::*;

#[test]
fn public_examples() {
    assert_eq!(cpu_sample_counts_total(&[]), 0);
    assert_eq!(cpu_sample_counts_total(&[4]), 1);
}
