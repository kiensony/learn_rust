use task_1831::*;

#[test]
fn public_examples() {
    assert_eq!(shard_occupancy_trace(&[]), vec![]);
    assert_eq!(shard_occupancy_trace(&[4]), vec![1u32]);
}
