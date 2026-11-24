use task_1602::*;

#[test]
fn public_examples() {
    assert_eq!(parity_property_total(&[]), 0);
    assert_eq!(parity_property_total(&[4]), 0);
}
