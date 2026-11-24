use task_1922::*;

#[test]
fn public_examples() {
    assert_eq!(four_slot_queue_total(&[]), 0);
    assert_eq!(four_slot_queue_total(&[4]), 4);
}
