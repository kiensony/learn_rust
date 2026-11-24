use task_1971::*;

#[test]
fn public_examples() {
    assert_eq!(atomic_toggle_trace(&[]), vec![]);
    assert_eq!(atomic_toggle_trace(&[4]), vec![0u32]);
}
