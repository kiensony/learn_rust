use task_2812::*;

#[test]
fn public_examples() {
    assert_eq!(service_validation_total(&[]), 0);
    assert_eq!(service_validation_total(&[4]), 1);
}
