use task_2345::*;

#[test]
fn public_examples() {
    assert_eq!(baseline_drift_changes(&[]), vec![]);
    assert_eq!(baseline_drift_changes(&[4]), vec![0u32]);
}
