use task_2885::*;

#[test]
fn public_examples() {
    assert_eq!(packed_pin_state_changes(&[]), vec![]);
    assert_eq!(packed_pin_state_changes(&[4]), vec![]);
}
