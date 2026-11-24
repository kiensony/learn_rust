use task_2255::*;

#[test]
fn public_examples() {
    assert_eq!(cpu_sample_counts_changes(&[]), vec![]);
    assert_eq!(cpu_sample_counts_changes(&[4]), vec![1u32]);
}
