use task_2975::*;

#[test]
fn public_examples() {
    assert_eq!(queue_depth_changes(&[]), vec![]);
    assert_eq!(queue_depth_changes(&[4]), vec![1u32]);
}
