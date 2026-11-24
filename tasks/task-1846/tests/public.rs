use task_1846::*;

#[test]
fn public_examples() {
    assert_eq!(
        shard_sums_threshold_crossing(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 0),
        Some((0, 9))
    );
    assert_eq!(shard_sums_threshold_crossing(&[], 0), None);
}
