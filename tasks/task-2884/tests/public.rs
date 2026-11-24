use task_2884::*;

#[test]
fn public_examples() {
    assert_eq!(packed_pin_state_peaks(&[]), vec![]);
    assert_eq!(packed_pin_state_peaks(&[4]), vec![]);
}
