use task_1409::*;

#[test]
fn public_examples() {
    assert_eq!(millisecond_remainder_longest_run(&[]), 0);
    assert_eq!(millisecond_remainder_longest_run(&[4]), 1);
}
