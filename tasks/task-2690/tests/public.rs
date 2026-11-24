use task_2690::*;

#[test]
fn public_examples() {
    assert_eq!(universal_slice_sum_reverse(&[]), vec![]);
    assert_eq!(universal_slice_sum_reverse(&[4]), vec![4u32]);
}
