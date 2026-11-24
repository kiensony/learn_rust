use task_2685::*;

#[test]
fn public_examples() {
    assert_eq!(universal_slice_sum_changes(&[]), vec![]);
    assert_eq!(universal_slice_sum_changes(&[4]), vec![4u32]);
}
