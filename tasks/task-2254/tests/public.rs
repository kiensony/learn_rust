use task_2254::*;

#[test]
fn public_examples() {
    assert_eq!(cpu_sample_counts_peaks(&[]), vec![]);
    assert_eq!(cpu_sample_counts_peaks(&[4]), vec![1u32]);
}
