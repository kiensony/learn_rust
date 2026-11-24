use task_2135::*;

#[test]
fn public_examples() {
    assert_eq!(histogram_bins_changes(&[]), vec![]);
    assert_eq!(histogram_bins_changes(&[4]), vec![1u32]);
}
