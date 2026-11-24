use task_1895::*;

#[test]
fn public_examples() {
    assert_eq!(weak_liveness_changes(&[]), vec![]);
    assert_eq!(weak_liveness_changes(&[4]), vec![0u32]);
}
