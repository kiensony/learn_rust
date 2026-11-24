use task_2132::*;

#[test]
fn public_examples() {
    assert_eq!(histogram_bins_total(&[]), 0);
    assert_eq!(histogram_bins_total(&[4]), 1);
}
