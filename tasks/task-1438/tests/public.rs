use task_1438::*;

#[test]
fn public_examples() {
    assert_eq!(timer_windows_steps(&[]), vec![]);
    assert_eq!(timer_windows_steps(&[4]), vec![]);
}
