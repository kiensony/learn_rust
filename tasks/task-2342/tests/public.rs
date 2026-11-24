use task_2342::*;

#[test]
fn public_examples() {
    assert_eq!(baseline_drift_total(&[]), 0);
    assert_eq!(baseline_drift_total(&[4]), 0);
}
