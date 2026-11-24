use task_1917::*;

#[test]
fn public_examples() {
    assert_eq!(two_slot_queue_distinct(&[]), vec![]);
    assert_eq!(two_slot_queue_distinct(&[4]), vec![4u32]);
}
