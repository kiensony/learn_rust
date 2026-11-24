use task_2059::*;

#[test]
fn public_examples() {
    assert_eq!(single_slot_receiver_longest_run(&[]), 0);
    assert_eq!(single_slot_receiver_longest_run(&[4]), 1);
}
