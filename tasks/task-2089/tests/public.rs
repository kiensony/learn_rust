use task_2089::*;

#[test]
fn public_examples() {
    assert_eq!(four_slot_intake_longest_run(&[]), 0);
    assert_eq!(four_slot_intake_longest_run(&[4]), 1);
}
