use task_2052::*;

#[test]
fn public_examples() {
    assert_eq!(single_slot_receiver_total(&[]), 0);
    assert_eq!(single_slot_receiver_total(&[4]), 4);
}
