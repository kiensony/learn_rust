use task_2972::*;

#[test]
fn public_examples() {
    assert_eq!(queue_depth_total(&[]), 0);
    assert_eq!(queue_depth_total(&[4]), 1);
}
