use task_1943::*;

#[test]
fn public_examples() {
    assert_eq!(locked_batch_peaks_bounds(&[]), None);
    assert_eq!(locked_batch_peaks_bounds(&[4]), Some((4u32, 4u32)));
}
