use task_2886::*;

#[test]
fn public_examples() {
    assert_eq!(packed_pin_state_descending_steps(&[]), 0);
    assert_eq!(packed_pin_state_descending_steps(&[4]), 0);
}
