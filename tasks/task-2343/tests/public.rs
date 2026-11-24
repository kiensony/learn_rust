use task_2343::*;

#[test]
fn public_examples() {
    assert_eq!(baseline_drift_bounds(&[]), None);
    assert_eq!(baseline_drift_bounds(&[4]), Some((0u32, 0u32)));
}
