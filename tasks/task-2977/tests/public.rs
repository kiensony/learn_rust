use task_2977::*;

#[test]
fn public_examples() {
    assert_eq!(queue_depth_distinct(&[]), vec![]);
    assert_eq!(queue_depth_distinct(&[4]), vec![1u32]);
}
