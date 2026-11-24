use task_2092::*;

#[test]
fn public_examples() {
    assert_eq!(five_slot_ingress_total(&[]), 0);
    assert_eq!(five_slot_ingress_total(&[4]), 4);
}
