use task_2806::*;

#[test]
fn public_examples() {
    assert_eq!(service_acknowledgments_descending_steps(&[]), 0);
    assert_eq!(service_acknowledgments_descending_steps(&[4]), 0);
}
