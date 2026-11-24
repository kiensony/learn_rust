use task_1928::*;

#[test]
fn public_examples() {
    assert_eq!(four_slot_queue_steps(&[]), vec![]);
    assert_eq!(four_slot_queue_steps(&[4]), vec![]);
}
