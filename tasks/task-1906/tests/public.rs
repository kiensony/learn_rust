use task_1906::*;

#[test]
fn public_examples() {
    assert_eq!(one_slot_queue_descending_steps(&[]), 0);
    assert_eq!(one_slot_queue_descending_steps(&[4]), 0);
}
