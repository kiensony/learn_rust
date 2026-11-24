use task_2074::*;

#[test]
fn public_examples() {
    assert_eq!(three_slot_mailbox_peaks(&[]), vec![]);
    assert_eq!(three_slot_mailbox_peaks(&[4]), vec![4u32]);
}
