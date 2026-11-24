use task_1606::*;

#[test]
fn public_examples() {
    assert_eq!(parity_property_descending_steps(&[]), 0);
    assert_eq!(parity_property_descending_steps(&[4]), 0);
}
