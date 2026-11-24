use task_1432::*;

#[test]
fn public_examples() {
    assert_eq!(timer_windows_total(&[]), 0);
    assert_eq!(timer_windows_total(&[4]), 1);
}
