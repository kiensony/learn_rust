use task_1974::*;

#[test]
fn public_examples() {
    assert_eq!(atomic_toggle_peaks(&[]), vec![]);
    assert_eq!(atomic_toggle_peaks(&[4]), vec![0u32]);
}
