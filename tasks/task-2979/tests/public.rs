use task_2979::*;

#[test]
fn public_examples() {
    assert_eq!(queue_depth_longest_run(&[]), 0);
    assert_eq!(queue_depth_longest_run(&[4]), 1);
}
