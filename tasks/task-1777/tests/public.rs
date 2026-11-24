use task_1777::*;

#[test]
fn public_examples() {
    assert_eq!(build_mask_distinct(&[]), vec![]);
    assert_eq!(build_mask_distinct(&[4]), vec![4u32]);
}
