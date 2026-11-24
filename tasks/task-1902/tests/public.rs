use task_1902::*;

#[test]
fn public_examples() {
    assert_eq!(one_slot_queue_total(&[]), 0);
    assert_eq!(one_slot_queue_total(&[4]), 4);
}
