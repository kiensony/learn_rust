use task_2087::*;

#[test]
fn public_examples() {
    assert_eq!(four_slot_intake_distinct(&[]), vec![]);
    assert_eq!(four_slot_intake_distinct(&[4]), vec![4u32]);
}
