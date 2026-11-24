use task_2082::*;

#[test]
fn public_examples() {
    assert_eq!(four_slot_intake_total(&[]), 0);
    assert_eq!(four_slot_intake_total(&[4]), 4);
}
