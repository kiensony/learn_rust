use task_1920::*;

#[test]
fn public_examples() {
    assert_eq!(two_slot_queue_reverse(&[]), vec![]);
    assert_eq!(two_slot_queue_reverse(&[4]), vec![4u32]);
}
