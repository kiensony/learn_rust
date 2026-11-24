use task_2883::*;

#[test]
fn public_examples() {
    assert_eq!(packed_pin_state_bounds(&[]), None);
    assert_eq!(packed_pin_state_bounds(&[4]), None);
}
