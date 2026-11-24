use task_1907::*;

#[test]
fn public_examples() {
    assert_eq!(one_slot_queue_distinct(&[]), vec![]);
    assert_eq!(one_slot_queue_distinct(&[4]), vec![4u32]);
}
