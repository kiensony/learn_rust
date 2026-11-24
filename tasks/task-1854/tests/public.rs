use task_1854::*;

#[test]
fn public_examples() {
    assert_eq!(shared_read_handles_peaks(&[]), vec![]);
    assert_eq!(shared_read_handles_peaks(&[4]), vec![1u32]);
}
