use task_1945::*;

#[test]
fn public_examples() {
    assert_eq!(locked_batch_peaks_changes(&[]), vec![]);
    assert_eq!(locked_batch_peaks_changes(&[4]), vec![4u32]);
}
