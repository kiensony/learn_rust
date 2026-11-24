use task_2890::*;

#[test]
fn public_examples() {
    assert_eq!(packed_pin_state_reverse(&[]), vec![]);
    assert_eq!(packed_pin_state_reverse(&[4]), vec![]);
}
