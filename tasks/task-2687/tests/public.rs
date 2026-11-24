use task_2687::*;

#[test]
fn public_examples() {
    assert_eq!(universal_slice_sum_distinct(&[]), vec![]);
    assert_eq!(universal_slice_sum_distinct(&[4]), vec![4u32]);
}
