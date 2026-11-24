use task_2081::*;

#[test]
fn public_examples() {
    assert_eq!(four_slot_intake_trace(&[]), vec![]);
    assert_eq!(four_slot_intake_trace(&[4]), vec![4u32]);
}
