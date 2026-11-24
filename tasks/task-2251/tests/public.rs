use task_2251::*;

#[test]
fn public_examples() {
    assert_eq!(cpu_sample_counts_trace(&[]), vec![]);
    assert_eq!(cpu_sample_counts_trace(&[4]), vec![1u32]);
}
