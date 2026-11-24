use task_2816::*;

#[test]
fn public_examples() {
    assert_eq!(service_validation_descending_steps(&[]), 0);
    assert_eq!(service_validation_descending_steps(&[4]), 0);
}
