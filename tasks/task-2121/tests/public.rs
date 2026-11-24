use task_2121::*;

#[test]
fn public_examples() {
    assert_eq!(queue_rotation_trace(&[]), vec![]);
    assert_eq!(queue_rotation_trace(&[4]), vec![4u32]);
}
