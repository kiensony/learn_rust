use task_1431::*;

#[test]
fn public_examples() {
    assert_eq!(timer_windows_trace(&[]), vec![]);
    assert_eq!(timer_windows_trace(&[4]), vec![1u32]);
}
