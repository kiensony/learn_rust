use task_1631::*;

#[test]
fn public_examples() {
    assert_eq!(clamp_property_trace(&[]), vec![]);
    assert_eq!(clamp_property_trace(&[4]), vec![0u32]);
}
