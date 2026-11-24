use task_1916::*;

#[test]
fn public_examples() {
    assert_eq!(two_slot_queue_descending_steps(&[]), 0);
    assert_eq!(two_slot_queue_descending_steps(&[4]), 0);
}
