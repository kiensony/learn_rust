use task_2995::*;

#[test]
fn public_examples() {
    assert_eq!(drain_order_changes(&[]), vec![]);
    assert_eq!(drain_order_changes(&[4]), vec![4u32]);
}
