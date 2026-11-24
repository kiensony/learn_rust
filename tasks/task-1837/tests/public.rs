use task_1837::*;

#[test]
fn public_examples() {
    assert_eq!(shard_occupancy_distinct(&[]), vec![]);
    assert_eq!(shard_occupancy_distinct(&[4]), vec![1u32]);
}
