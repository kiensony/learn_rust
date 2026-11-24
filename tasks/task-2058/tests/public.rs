use task_2058::*;

#[test]
fn public_examples() {
    assert_eq!(single_slot_receiver_steps(&[]), vec![]);
    assert_eq!(single_slot_receiver_steps(&[4]), vec![]);
}
