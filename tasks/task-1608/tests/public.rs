use task_1608::*;

#[test]
fn public_examples() {
    assert_eq!(parity_property_steps(&[]), vec![]);
    assert_eq!(parity_property_steps(&[4]), vec![]);
}
