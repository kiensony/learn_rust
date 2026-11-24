use task_1848::*;

#[test]
fn public_examples() {
    assert_eq!(
        shard_masks_rolling_load(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 1),
        vec![15u64, 31u64, 31u64]
    );
    assert_eq!(shard_masks_rolling_load(&[], 0), vec![]);
}
