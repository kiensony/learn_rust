use task_2994::*;

#[test]
fn public_examples() {
    assert_eq!(drain_order_peaks(&[]), vec![]);
    assert_eq!(drain_order_peaks(&[4]), vec![4u32]);
}
