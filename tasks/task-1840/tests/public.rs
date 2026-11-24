use task_1840::*;

#[test]
fn public_examples() {
    assert_eq!(shard_occupancy_reverse(&[]), vec![]);
    assert_eq!(shard_occupancy_reverse(&[4]), vec![1u32]);
}
