use task_2921::*;

#[test]
fn public_examples() {
    assert_eq!(failure_threshold_trace(&[]), vec![]);
    assert_eq!(failure_threshold_trace(&[4]), vec![]);
}
