use task_2992::*;

#[test]
fn public_examples() {
    assert_eq!(drain_order_total(&[]), 0);
    assert_eq!(drain_order_total(&[4]), 4);
}
