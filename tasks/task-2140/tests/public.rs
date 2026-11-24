use task_2140::*;

#[test]
fn public_examples() {
    assert_eq!(histogram_bins_reverse(&[]), vec![]);
    assert_eq!(histogram_bins_reverse(&[4]), vec![1u32]);
}
