use task_2009::*;

#[test]
fn public_examples() {
    assert_eq!(polled_lengths_longest_run(&[]), 0);
    assert_eq!(polled_lengths_longest_run(&[4]), 1);
}
