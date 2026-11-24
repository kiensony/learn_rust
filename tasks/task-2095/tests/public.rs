use task_2095::*;

#[test]
fn public_examples() {
    assert_eq!(five_slot_ingress_changes(&[]), vec![]);
    assert_eq!(five_slot_ingress_changes(&[4]), vec![4u32]);
}
