use task_2632::*;

#[test]
fn public_examples() {
    assert_eq!(embedded_saturation_total(&[]), 0);
    assert_eq!(embedded_saturation_total(&[4]), 8);
}
