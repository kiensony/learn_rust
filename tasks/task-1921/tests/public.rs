use task_1921::*;

#[test]
fn public_examples() {
    assert_eq!(four_slot_queue_trace(&[]), vec![]);
    assert_eq!(four_slot_queue_trace(&[4]), vec![4u32]);
}
