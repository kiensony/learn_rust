use task_1829::*;

#[test]
fn public_examples() {
    assert_eq!(shard_masks_longest_run(&[]), 0);
    assert_eq!(shard_masks_longest_run(&[4]), 1);
}
