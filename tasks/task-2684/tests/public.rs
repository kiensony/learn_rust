use task_2684::*;

#[test]
fn public_examples() {
    assert_eq!(universal_slice_sum_peaks(&[]), vec![]);
    assert_eq!(universal_slice_sum_peaks(&[4]), vec![4u32]);
}
