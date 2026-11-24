use task_2072::*;

#[test]
fn public_examples() {
    assert_eq!(three_slot_mailbox_total(&[]), 0);
    assert_eq!(three_slot_mailbox_total(&[4]), 4);
}
