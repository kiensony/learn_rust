use task_2080::*;

#[test]
fn public_examples() {
    assert_eq!(three_slot_mailbox_reverse(&[]), vec![]);
    assert_eq!(three_slot_mailbox_reverse(&[4]), vec![4u32]);
}
