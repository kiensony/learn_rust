use task_2973::*;

#[test]
fn public_examples() {
    assert_eq!(queue_depth_bounds(&[]), None);
    assert_eq!(queue_depth_bounds(&[4]), Some((1u32, 1u32)));
}
