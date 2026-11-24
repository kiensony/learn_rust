use task_2126::*;

#[test]
fn public_examples() {
    assert_eq!(queue_rotation_descending_steps(&[]), 0);
    assert_eq!(queue_rotation_descending_steps(&[4]), 0);
}
