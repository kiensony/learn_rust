use task_1830::*;

#[test]
fn public_examples() {
    assert_eq!(shard_masks_reverse(&[]), vec![]);
    assert_eq!(shard_masks_reverse(&[4]), vec![4u32]);
}
