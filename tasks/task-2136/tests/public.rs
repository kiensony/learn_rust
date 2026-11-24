use task_2136::*;

#[test]
fn public_examples() {
    assert_eq!(histogram_bins_descending_steps(&[]), 0);
    assert_eq!(histogram_bins_descending_steps(&[4]), 0);
}
