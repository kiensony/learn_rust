use task_2882::*;

#[test]
fn public_examples() {
    assert_eq!(packed_pin_state_total(&[]), 0);
    assert_eq!(packed_pin_state_total(&[4]), 0);
}
