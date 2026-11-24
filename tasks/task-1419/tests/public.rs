use task_1419::*;

#[test]
fn public_examples() {
    assert_eq!(elapsed_seconds_longest_run(&[]), 0);
    assert_eq!(elapsed_seconds_longest_run(&[4]), 1);
}
