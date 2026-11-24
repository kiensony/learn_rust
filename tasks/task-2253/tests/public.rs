use task_2253::*;

#[test]
fn public_examples() {
    assert_eq!(cpu_sample_counts_bounds(&[]), None);
    assert_eq!(cpu_sample_counts_bounds(&[4]), Some((1u32, 1u32)));
}
