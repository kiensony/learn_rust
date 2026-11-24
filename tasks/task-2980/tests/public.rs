use task_2980::*;

#[test]
fn public_examples() {
    assert_eq!(queue_depth_reverse(&[]), vec![]);
    assert_eq!(queue_depth_reverse(&[4]), vec![1u32]);
}
