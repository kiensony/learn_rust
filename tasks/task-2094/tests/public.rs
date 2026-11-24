use task_2094::*;

#[test]
fn public_examples() {
    assert_eq!(five_slot_ingress_peaks(&[]), vec![]);
    assert_eq!(five_slot_ingress_peaks(&[4]), vec![4u32]);
}
