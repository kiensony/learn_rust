use task_1925::*;

#[test]
fn public_examples() {
    assert_eq!(four_slot_queue_changes(&[]), vec![]);
    assert_eq!(four_slot_queue_changes(&[4]), vec![4u32]);
}
