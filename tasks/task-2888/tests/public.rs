use task_2888::*;

#[test]
fn public_examples() {
    assert_eq!(packed_pin_state_steps(&[]), vec![]);
    assert_eq!(packed_pin_state_steps(&[4]), vec![]);
}
