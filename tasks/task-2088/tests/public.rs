use task_2088::*;

#[test]
fn public_examples() {
    assert_eq!(four_slot_intake_steps(&[]), vec![]);
    assert_eq!(four_slot_intake_steps(&[4]), vec![]);
}
