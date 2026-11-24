use task_1905::*;

#[test]
fn public_examples() {
    assert_eq!(one_slot_queue_changes(&[]), vec![]);
    assert_eq!(one_slot_queue_changes(&[4]), vec![4u32]);
}
