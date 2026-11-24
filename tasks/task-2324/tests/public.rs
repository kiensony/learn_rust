use task_2324::*;

#[test]
fn public_examples() {
    assert_eq!(batch_medians_peaks(&[]), vec![]);
    assert_eq!(batch_medians_peaks(&[4]), vec![4u32]);
}
