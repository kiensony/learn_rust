use task_1821::*;

#[test]
fn public_examples() {
    assert_eq!(shard_masks_trace(&[]), vec![]);
    assert_eq!(shard_masks_trace(&[4]), vec![4u32]);
}
