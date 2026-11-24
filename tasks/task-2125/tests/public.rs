use task_2125::*;

#[test]
fn public_examples() {
    assert_eq!(queue_rotation_changes(&[]), vec![]);
    assert_eq!(queue_rotation_changes(&[4]), vec![4u32]);
}
