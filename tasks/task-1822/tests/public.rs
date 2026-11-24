use task_1822::*;

#[test]
fn public_examples() {
    assert_eq!(shard_masks_total(&[]), 0);
    assert_eq!(shard_masks_total(&[4]), 4);
}
