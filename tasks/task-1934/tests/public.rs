use task_1934::*;

#[test]
fn public_examples() {
    assert_eq!(locked_batch_totals_peaks(&[]), vec![]);
    assert_eq!(locked_batch_totals_peaks(&[4]), vec![4u32]);
}
