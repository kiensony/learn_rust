use task_2084::*;

#[test]
fn public_examples() {
    assert_eq!(four_slot_intake_peaks(&[]), vec![]);
    assert_eq!(four_slot_intake_peaks(&[4]), vec![4u32]);
}
