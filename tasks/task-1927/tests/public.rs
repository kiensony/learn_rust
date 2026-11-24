use task_1927::*;

#[test]
fn public_examples() {
    assert_eq!(four_slot_queue_distinct(&[]), vec![]);
    assert_eq!(four_slot_queue_distinct(&[4]), vec![4u32]);
}
