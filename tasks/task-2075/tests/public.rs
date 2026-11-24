use task_2075::*;

#[test]
fn public_examples() {
    assert_eq!(three_slot_mailbox_changes(&[]), vec![]);
    assert_eq!(three_slot_mailbox_changes(&[4]), vec![4u32]);
}
