use task_1911::*;

#[test]
fn public_examples() {
    assert_eq!(two_slot_queue_trace(&[]), vec![]);
    assert_eq!(two_slot_queue_trace(&[4]), vec![4u32]);
}
