use task_1835::*;

#[test]
fn public_examples() {
    assert_eq!(shard_occupancy_changes(&[]), vec![]);
    assert_eq!(shard_occupancy_changes(&[4]), vec![1u32]);
}
