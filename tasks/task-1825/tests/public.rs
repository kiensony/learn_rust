use task_1825::*;

#[test]
fn public_examples() {
    assert_eq!(shard_masks_changes(&[]), vec![]);
    assert_eq!(shard_masks_changes(&[4]), vec![4u32]);
}
