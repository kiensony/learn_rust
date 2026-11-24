use task_1912::*;

#[test]
fn public_examples() {
    assert_eq!(two_slot_queue_total(&[]), 0);
    assert_eq!(two_slot_queue_total(&[4]), 4);
}
