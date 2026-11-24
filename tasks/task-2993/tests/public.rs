use task_2993::*;

#[test]
fn public_examples() {
    assert_eq!(drain_order_bounds(&[]), None);
    assert_eq!(drain_order_bounds(&[4]), Some((4u32, 4u32)));
}
