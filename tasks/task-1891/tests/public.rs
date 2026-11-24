use task_1891::*;

#[test]
fn public_examples() {
    assert_eq!(weak_liveness_trace(&[]), vec![]);
    assert_eq!(weak_liveness_trace(&[4]), vec![0u32]);
}
