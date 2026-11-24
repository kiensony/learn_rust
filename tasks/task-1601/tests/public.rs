use task_1601::*;

#[test]
fn public_examples() {
    assert_eq!(parity_property_trace(&[]), vec![]);
    assert_eq!(parity_property_trace(&[4]), vec![0u32]);
}
