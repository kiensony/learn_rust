use task_1944::*;

#[test]
fn public_examples() {
    assert_eq!(locked_batch_peaks_peaks(&[]), vec![]);
    assert_eq!(locked_batch_peaks_peaks(&[4]), vec![4u32]);
}
