use task_1824::*;

#[test]
fn public_examples() {
    assert_eq!(shard_masks_peaks(&[]), vec![]);
    assert_eq!(shard_masks_peaks(&[4]), vec![4u32]);
}
