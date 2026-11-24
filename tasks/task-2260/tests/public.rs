use task_2260::*;

#[test]
fn public_examples() {
    assert_eq!(cpu_sample_counts_reverse(&[]), vec![]);
    assert_eq!(cpu_sample_counts_reverse(&[4]), vec![1u32]);
}
