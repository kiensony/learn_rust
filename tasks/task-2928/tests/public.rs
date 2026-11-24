use task_2928::*;

#[test]
fn public_examples() {
    assert_eq!(failure_threshold_steps(&[]), vec![]);
    assert_eq!(failure_threshold_steps(&[4]), vec![]);
}
