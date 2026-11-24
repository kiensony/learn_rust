use task_1864::*;

#[test]
fn public_examples() {
    assert_eq!(weak_read_handles_peaks(&[]), vec![]);
    assert_eq!(weak_read_handles_peaks(&[4]), vec![0u32]);
}
