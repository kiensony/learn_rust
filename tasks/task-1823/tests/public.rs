use task_1823::*;

#[test]
fn public_examples() {
    assert_eq!(shard_masks_bounds(&[]), None);
    assert_eq!(shard_masks_bounds(&[4]), Some((4u32, 4u32)));
}
