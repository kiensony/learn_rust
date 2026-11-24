use task_2071::*;

#[test]
fn public_examples() {
    assert_eq!(three_slot_mailbox_trace(&[]), vec![]);
    assert_eq!(three_slot_mailbox_trace(&[4]), vec![4u32]);
}
