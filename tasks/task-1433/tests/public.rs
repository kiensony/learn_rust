use task_1433::*;

#[test]
fn public_examples() {
    assert_eq!(timer_windows_bounds(&[]), None);
    assert_eq!(timer_windows_bounds(&[4]), Some((1u32, 1u32)));
}
