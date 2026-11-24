use task_2139::*;

#[test]
fn public_examples() {
    assert_eq!(histogram_bins_longest_run(&[]), 0);
    assert_eq!(histogram_bins_longest_run(&[4]), 1);
}
