use task_2926::*;

#[test]
fn public_examples() {
    assert_eq!(failure_threshold_descending_steps(&[]), 0);
    assert_eq!(failure_threshold_descending_steps(&[4]), 0);
}
