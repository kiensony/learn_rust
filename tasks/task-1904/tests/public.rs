use task_1904::*;

#[test]
fn public_examples() {
    assert_eq!(one_slot_queue_peaks(&[]), vec![]);
    assert_eq!(one_slot_queue_peaks(&[4]), vec![4u32]);
}
