use task_1780::*;

#[test]
fn public_examples() {
    assert_eq!(build_mask_reverse(&[]), vec![]);
    assert_eq!(build_mask_reverse(&[4]), vec![4u32]);
}
