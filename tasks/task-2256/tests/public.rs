use task_2256::*;

#[test]
fn public_examples() {
    assert_eq!(cpu_sample_counts_descending_steps(&[]), 0);
    assert_eq!(cpu_sample_counts_descending_steps(&[4]), 0);
}
