use task_2978::*;

#[test]
fn public_examples() {
    assert_eq!(queue_depth_steps(&[]), vec![]);
    assert_eq!(queue_depth_steps(&[4]), vec![]);
}
