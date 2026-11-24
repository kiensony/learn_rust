use task_2093::*;

#[test]
fn public_examples() {
    assert_eq!(five_slot_ingress_bounds(&[]), None);
    assert_eq!(five_slot_ingress_bounds(&[4]), Some((4u32, 4u32)));
}
