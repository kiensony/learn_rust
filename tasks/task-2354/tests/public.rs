use task_2354::*;

#[test]
fn public_examples() {
    assert_eq!(single_pass_prefix_maximum_peaks(&[]), vec![]);
    assert_eq!(single_pass_prefix_maximum_peaks(&[4]), vec![4u32]);
}
