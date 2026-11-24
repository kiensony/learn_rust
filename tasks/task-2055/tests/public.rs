use task_2055::*;

#[test]
fn public_examples() {
    assert_eq!(single_slot_receiver_changes(&[]), vec![]);
    assert_eq!(single_slot_receiver_changes(&[4]), vec![4u32]);
}
