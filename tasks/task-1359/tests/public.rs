use task_1359::*;

#[test]
fn public_examples() {
    assert_eq!(frame_lengths_longest_run(&[]), 0);
    assert_eq!(frame_lengths_longest_run(&[0]), 1);
}
