use task_2134::*;

#[test]
fn public_examples() {
    assert_eq!(histogram_bins_peaks(&[]), vec![]);
    assert_eq!(histogram_bins_peaks(&[4]), vec![1u32]);
}
