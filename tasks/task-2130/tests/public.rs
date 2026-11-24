use task_2130::*;

#[test]
fn public_examples() {
    assert_eq!(queue_rotation_reverse(&[]), vec![]);
    assert_eq!(queue_rotation_reverse(&[4]), vec![4u32]);
}
