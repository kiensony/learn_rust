use task_1849::*;

#[test]
fn transfer_case() {
    assert_eq!(
        shard_occupancy_frequent_readings(&[9, 2, 2, 8, 0, 5, 5, 0, 8, 2, 2, 9], 2),
        vec![4u32, 2u32]
    );
}
