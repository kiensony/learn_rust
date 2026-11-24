use task_1632::*;

#[test]
fn public_examples() {
    assert_eq!(clamp_property_total(&[]), 0);
    assert_eq!(clamp_property_total(&[4]), 0);
}
