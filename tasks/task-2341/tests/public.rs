use task_2341::*;

#[test]
fn public_examples() {
    assert_eq!(baseline_drift_trace(&[]), vec![]);
    assert_eq!(baseline_drift_trace(&[4]), vec![0u32]);
}
