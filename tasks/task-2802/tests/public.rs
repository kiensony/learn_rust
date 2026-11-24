use task_2802::*;

#[test]
fn public_examples() {
    assert_eq!(service_acknowledgments_total(&[]), 0);
    assert_eq!(service_acknowledgments_total(&[4]), 4);
}
