use task_1914::*;

#[test]
fn public_examples() {
    assert_eq!(two_slot_queue_peaks(&[]), vec![]);
    assert_eq!(two_slot_queue_peaks(&[4]), vec![4u32]);
}
