use task_2138::*;

#[test]
fn public_examples() {
    assert_eq!(histogram_bins_steps(&[]), vec![]);
    assert_eq!(histogram_bins_steps(&[4]), vec![]);
}
