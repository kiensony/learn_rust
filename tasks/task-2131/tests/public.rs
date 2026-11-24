use task_2131::*;

#[test]
fn public_examples() {
    assert_eq!(histogram_bins_trace(&[]), vec![]);
    assert_eq!(histogram_bins_trace(&[4]), vec![1u32]);
}
