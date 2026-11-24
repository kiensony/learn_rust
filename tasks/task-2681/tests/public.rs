use task_2681::*;

#[test]
fn public_examples() {
    assert_eq!(universal_slice_sum_trace(&[]), vec![]);
    assert_eq!(universal_slice_sum_trace(&[4]), vec![4u32]);
}
