use task_1775::*;

#[test]
fn public_examples() {
    assert_eq!(build_mask_changes(&[]), vec![]);
    assert_eq!(build_mask_changes(&[4]), vec![4u32]);
}
