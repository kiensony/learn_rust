use task_2069::*;

#[test]
fn public_examples() {
    assert_eq!(double_buffer_receiver_longest_run(&[]), 0);
    assert_eq!(double_buffer_receiver_longest_run(&[4]), 1);
}
