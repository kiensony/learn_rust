use task_2334::*;

#[test]
fn public_examples() {
    assert_eq!(batch_variation_peaks(&[]), vec![]);
    assert_eq!(batch_variation_peaks(&[4]), vec![0u32]);
}
