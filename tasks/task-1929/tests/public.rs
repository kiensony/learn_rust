use task_1929::*;

#[test]
fn public_examples() {
    assert_eq!(four_slot_queue_longest_run(&[]), 0);
    assert_eq!(four_slot_queue_longest_run(&[4]), 1);
}
