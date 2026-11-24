use task_2398::*;

#[test]
fn public_examples() {
    assert_eq!(
        cumulative_occupancy_rolling_load(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 1),
        vec![1u64, 2u64, 3u64, 4u64, 5u64, 6u64, 6u64, 7u64, 8u64, 9u64]
    );
    assert_eq!(cumulative_occupancy_rolling_load(&[], 0), vec![]);
}
