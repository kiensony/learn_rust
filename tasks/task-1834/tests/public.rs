use task_1834::*;

#[test]
fn public_examples() {
    assert_eq!(shard_occupancy_peaks(&[]), vec![]);
    assert_eq!(shard_occupancy_peaks(&[4]), vec![1u32]);
}
