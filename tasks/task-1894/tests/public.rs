use task_1894::*;

#[test]
fn public_examples() {
    assert_eq!(weak_liveness_peaks(&[]), vec![]);
    assert_eq!(weak_liveness_peaks(&[4]), vec![0u32]);
}
