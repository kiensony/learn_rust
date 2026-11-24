use task_1439::*;

#[test]
fn public_examples() {
    assert_eq!(timer_windows_longest_run(&[]), 0);
    assert_eq!(timer_windows_longest_run(&[4]), 1);
}
