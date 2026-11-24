use task_1975::*;

#[test]
fn public_examples() {
    assert_eq!(atomic_toggle_changes(&[]), vec![]);
    assert_eq!(atomic_toggle_changes(&[4]), vec![0u32]);
}
