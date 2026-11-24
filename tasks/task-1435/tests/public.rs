use task_1435::*;

#[test]
fn public_examples() {
    assert_eq!(timer_windows_changes(&[]), vec![]);
    assert_eq!(timer_windows_changes(&[4]), vec![1u32]);
}
