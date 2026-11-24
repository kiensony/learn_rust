use task_2974::*;

#[test]
fn public_examples() {
    assert_eq!(queue_depth_peaks(&[]), vec![]);
    assert_eq!(queue_depth_peaks(&[4]), vec![1u32]);
}
