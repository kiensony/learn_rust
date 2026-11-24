use task_1924::*;

#[test]
fn public_examples() {
    assert_eq!(four_slot_queue_peaks(&[]), vec![]);
    assert_eq!(four_slot_queue_peaks(&[4]), vec![4u32]);
}
