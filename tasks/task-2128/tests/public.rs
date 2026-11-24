use task_2128::*;

#[test]
fn public_examples() {
    assert_eq!(queue_rotation_steps(&[]), vec![]);
    assert_eq!(queue_rotation_steps(&[4]), vec![]);
}
