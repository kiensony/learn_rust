use task_2991::*;

#[test]
fn public_examples() {
    assert_eq!(drain_order_trace(&[]), vec![]);
    assert_eq!(drain_order_trace(&[4]), vec![4u32]);
}
