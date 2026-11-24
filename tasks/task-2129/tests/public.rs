use task_2129::*;

#[test]
fn public_examples() {
    assert_eq!(queue_rotation_longest_run(&[]), 0);
    assert_eq!(queue_rotation_longest_run(&[4]), 1);
}
