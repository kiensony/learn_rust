use task_1942::*;

#[test]
fn public_examples() {
    assert_eq!(locked_batch_peaks_total(&[]), 0);
    assert_eq!(locked_batch_peaks_total(&[4]), 4);
}
