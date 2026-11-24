use task_1941::*;

#[test]
fn public_examples() {
    assert_eq!(locked_batch_peaks_trace(&[]), vec![]);
    assert_eq!(locked_batch_peaks_trace(&[4]), vec![4u32]);
}
