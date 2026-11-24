use task_2887::*;

#[test]
fn public_examples() {
    assert_eq!(packed_pin_state_distinct(&[]), vec![]);
    assert_eq!(packed_pin_state_distinct(&[4]), vec![]);
}
