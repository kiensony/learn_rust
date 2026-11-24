use task_2073::*;

#[test]
fn public_examples() {
    assert_eq!(three_slot_mailbox_bounds(&[]), None);
    assert_eq!(three_slot_mailbox_bounds(&[4]), Some((4u32, 4u32)));
}
