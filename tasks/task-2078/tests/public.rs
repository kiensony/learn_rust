use task_2078::*;

#[test]
fn public_examples() {
    assert_eq!(three_slot_mailbox_steps(&[]), vec![]);
    assert_eq!(three_slot_mailbox_steps(&[4]), vec![]);
}
