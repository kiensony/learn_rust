use task_2881::*;

#[test]
fn public_examples() {
    assert_eq!(packed_pin_state_trace(&[]), vec![]);
    assert_eq!(packed_pin_state_trace(&[4]), vec![]);
}
