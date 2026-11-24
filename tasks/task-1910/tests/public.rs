use task_1910::*;

#[test]
fn public_examples() {
    assert_eq!(one_slot_queue_reverse(&[]), vec![]);
    assert_eq!(one_slot_queue_reverse(&[4]), vec![4u32]);
}
