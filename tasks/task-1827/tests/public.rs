use task_1827::*;

#[test]
fn public_examples() {
    assert_eq!(shard_masks_distinct(&[]), vec![]);
    assert_eq!(shard_masks_distinct(&[4]), vec![4u32]);
}
