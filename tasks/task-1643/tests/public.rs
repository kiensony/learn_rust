use task_1643::*;

#[test]
fn public_examples() {
    assert_eq!(shift_property_bounds(&[]), None);
    assert_eq!(shift_property_bounds(&[4]), Some((0u32, 0u32)));
}
