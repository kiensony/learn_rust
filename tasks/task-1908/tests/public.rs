use task_1908::*;

#[test]
fn public_examples() {
    assert_eq!(one_slot_queue_steps(&[]), vec![]);
    assert_eq!(one_slot_queue_steps(&[4]), vec![]);
}
