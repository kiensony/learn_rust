use task_2123::*;

#[test]
fn public_examples() {
    assert_eq!(queue_rotation_bounds(&[]), None);
    assert_eq!(queue_rotation_bounds(&[4]), Some((4u32, 4u32)));
}
