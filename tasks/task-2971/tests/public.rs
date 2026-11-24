use task_2971::*;

#[test]
fn public_examples() {
    assert_eq!(queue_depth_trace(&[]), vec![]);
    assert_eq!(queue_depth_trace(&[4]), vec![1u32]);
}
