use task_2137::*;

#[test]
fn public_examples() {
    assert_eq!(histogram_bins_distinct(&[]), vec![]);
    assert_eq!(histogram_bins_distinct(&[4]), vec![1u32]);
}
