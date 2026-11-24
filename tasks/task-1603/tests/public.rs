use task_1603::*;

#[test]
fn public_examples() {
    assert_eq!(parity_property_bounds(&[]), None);
    assert_eq!(parity_property_bounds(&[4]), Some((0u32, 0u32)));
}
