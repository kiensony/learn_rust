use task_1434::*;

#[test]
fn public_examples() {
    assert_eq!(timer_windows_peaks(&[]), vec![]);
    assert_eq!(timer_windows_peaks(&[4]), vec![1u32]);
}
