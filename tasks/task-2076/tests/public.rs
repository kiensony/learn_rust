use task_2076::*;

#[test]
fn public_examples() {
    assert_eq!(three_slot_mailbox_descending_steps(&[]), 0);
    assert_eq!(three_slot_mailbox_descending_steps(&[4]), 0);
}
