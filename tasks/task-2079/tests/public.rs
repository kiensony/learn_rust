use task_2079::*;

#[test]
fn public_examples() {
    assert_eq!(three_slot_mailbox_longest_run(&[]), 0);
    assert_eq!(three_slot_mailbox_longest_run(&[4]), 1);
}
