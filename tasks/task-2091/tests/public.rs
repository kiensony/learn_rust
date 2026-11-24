use task_2091::*;

#[test]
fn public_examples() {
    assert_eq!(five_slot_ingress_trace(&[]), vec![]);
    assert_eq!(five_slot_ingress_trace(&[4]), vec![4u32]);
}
