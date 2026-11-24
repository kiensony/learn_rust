use task_1828::*;

#[test]
fn public_examples() {
    assert_eq!(shard_masks_steps(&[]), vec![]);
    assert_eq!(shard_masks_steps(&[4]), vec![]);
}
