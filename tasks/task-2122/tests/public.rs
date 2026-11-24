use task_2122::*;

#[test]
fn public_examples() {
    assert_eq!(queue_rotation_total(&[]), 0);
    assert_eq!(queue_rotation_total(&[4]), 4);
}
