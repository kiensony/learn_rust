use task_1909::*;

#[test]
fn public_examples() {
    assert_eq!(one_slot_queue_longest_run(&[]), 0);
    assert_eq!(one_slot_queue_longest_run(&[4]), 1);
}
