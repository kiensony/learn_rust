use task_1901::*;

#[test]
fn public_examples() {
    assert_eq!(one_slot_queue_trace(&[]), vec![]);
    assert_eq!(one_slot_queue_trace(&[4]), vec![4u32]);
}
