use task_2922::*;

#[test]
fn public_examples() {
    assert_eq!(failure_threshold_total(&[]), 0);
    assert_eq!(failure_threshold_total(&[4]), 0);
}
