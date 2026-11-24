use task_1849::*;

#[test]
fn public_examples() {
    assert_eq!(
        shard_occupancy_frequent_readings(&[9, 2, 2, 8, 0, 5], 3),
        vec![4u32, 1u32]
    );
    assert_eq!(shard_occupancy_frequent_readings(&[], 0), vec![]);
}
