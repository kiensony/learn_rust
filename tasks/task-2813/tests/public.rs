use task_2813::*;

#[test]
fn public_examples() {
    assert_eq!(service_validation_bounds(&[]), None);
    assert_eq!(service_validation_bounds(&[4]), Some((1u32, 1u32)));
}
