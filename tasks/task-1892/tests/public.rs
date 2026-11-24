use task_1892::*;

#[test]
fn public_examples() {
    assert_eq!(weak_liveness_total(&[]), 0);
    assert_eq!(weak_liveness_total(&[4]), 0);
}
