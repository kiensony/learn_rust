use task_2934::*;

#[test]
fn public_examples() {
    assert_eq!(stability_band_peaks(&[]), vec![]);
    assert_eq!(stability_band_peaks(&[4]), vec![]);
}
