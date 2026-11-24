use task_1636::*;

#[test]
fn public_examples() {
    assert_eq!(clamp_property_descending_steps(&[]), 0);
    assert_eq!(clamp_property_descending_steps(&[4]), 0);
}
