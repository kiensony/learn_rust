use task_2636::*;

#[test]
fn public_examples() {
    assert_eq!(embedded_saturation_descending_steps(&[]), 0);
    assert_eq!(embedded_saturation_descending_steps(&[4]), 0);
}
