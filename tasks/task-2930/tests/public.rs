use task_2930::*;

#[test]
fn public_examples() {
    assert_eq!(failure_threshold_reverse(&[]), vec![]);
    assert_eq!(failure_threshold_reverse(&[4]), vec![]);
}
