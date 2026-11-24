use task_2925::*;

#[test]
fn public_examples() {
    assert_eq!(failure_threshold_changes(&[]), vec![]);
    assert_eq!(failure_threshold_changes(&[4]), vec![]);
}
