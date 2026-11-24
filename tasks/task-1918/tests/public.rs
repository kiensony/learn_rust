use task_1918::*;

#[test]
fn public_examples() {
    assert_eq!(two_slot_queue_steps(&[]), vec![]);
    assert_eq!(two_slot_queue_steps(&[4]), vec![]);
}
