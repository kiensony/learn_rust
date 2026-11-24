use task_2364::*;

#[test]
fn public_examples() {
    assert_eq!(single_pass_prefix_minimum_peaks(&[]), vec![]);
    assert_eq!(single_pass_prefix_minimum_peaks(&[4]), vec![4u32]);
}
