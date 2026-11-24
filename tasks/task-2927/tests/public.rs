use task_2927::*;

#[test]
fn public_examples() {
    assert_eq!(failure_threshold_distinct(&[]), vec![]);
    assert_eq!(failure_threshold_distinct(&[4]), vec![]);
}
