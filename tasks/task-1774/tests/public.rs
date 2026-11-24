use task_1774::*;

#[test]
fn public_examples() {
    assert_eq!(build_mask_peaks(&[]), vec![]);
    assert_eq!(build_mask_peaks(&[4]), vec![4u32]);
}
