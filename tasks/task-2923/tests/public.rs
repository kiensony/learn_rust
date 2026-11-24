use task_2923::*;

#[test]
fn public_examples() {
    assert_eq!(failure_threshold_bounds(&[]), None);
    assert_eq!(failure_threshold_bounds(&[4]), None);
}
