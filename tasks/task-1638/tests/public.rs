use task_1638::*;

#[test]
fn public_examples() {
    assert_eq!(clamp_property_steps(&[]), vec![]);
    assert_eq!(clamp_property_steps(&[4]), vec![]);
}
