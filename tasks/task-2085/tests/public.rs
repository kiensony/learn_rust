use task_2085::*;

#[test]
fn public_examples() {
    assert_eq!(four_slot_intake_changes(&[]), vec![]);
    assert_eq!(four_slot_intake_changes(&[4]), vec![4u32]);
}
