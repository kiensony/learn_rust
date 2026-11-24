use task_2258::*;

#[test]
fn public_examples() {
    assert_eq!(cpu_sample_counts_steps(&[]), vec![]);
    assert_eq!(cpu_sample_counts_steps(&[4]), vec![]);
}
