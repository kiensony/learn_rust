use task_2257::*;

#[test]
fn public_examples() {
    assert_eq!(cpu_sample_counts_distinct(&[]), vec![]);
    assert_eq!(cpu_sample_counts_distinct(&[4]), vec![1u32]);
}
