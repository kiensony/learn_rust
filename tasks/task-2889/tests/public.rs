use task_2889::*;

#[test]
fn public_examples() {
    assert_eq!(packed_pin_state_longest_run(&[]), 0);
    assert_eq!(packed_pin_state_longest_run(&[4]), 0);
}
