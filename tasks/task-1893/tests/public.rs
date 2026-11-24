use task_1893::*;

#[test]
fn public_examples() {
    assert_eq!(weak_liveness_bounds(&[]), None);
    assert_eq!(weak_liveness_bounds(&[4]), Some((0u32, 0u32)));
}
