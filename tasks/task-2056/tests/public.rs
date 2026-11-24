use task_2056::*;

#[test]
fn public_examples() {
    assert_eq!(single_slot_receiver_descending_steps(&[]), 0);
    assert_eq!(single_slot_receiver_descending_steps(&[4]), 0);
}
