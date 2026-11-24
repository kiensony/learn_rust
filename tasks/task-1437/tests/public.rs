use task_1437::*;

#[test]
fn public_examples() {
    assert_eq!(timer_windows_distinct(&[]), vec![]);
    assert_eq!(timer_windows_distinct(&[4]), vec![1u32]);
}
