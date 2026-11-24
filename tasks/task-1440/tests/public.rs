use task_1440::*;

#[test]
fn public_examples() {
    assert_eq!(timer_windows_reverse(&[]), vec![]);
    assert_eq!(timer_windows_reverse(&[4]), vec![1u32]);
}
