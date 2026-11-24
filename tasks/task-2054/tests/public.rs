use task_2054::*;

#[test]
fn public_examples() {
    assert_eq!(single_slot_receiver_peaks(&[]), vec![]);
    assert_eq!(single_slot_receiver_peaks(&[4]), vec![4u32]);
}
