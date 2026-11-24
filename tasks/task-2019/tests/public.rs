use task_2019::*;

#[test]
fn public_examples() {
    assert_eq!(polled_flags_longest_run(&[]), 0);
    assert_eq!(polled_flags_longest_run(&[4]), 1);
}
