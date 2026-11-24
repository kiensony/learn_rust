use task_1930::*;

#[test]
fn public_examples() {
    assert_eq!(four_slot_queue_reverse(&[]), vec![]);
    assert_eq!(four_slot_queue_reverse(&[4]), vec![4u32]);
}
