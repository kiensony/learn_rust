use task_1832::*;

#[test]
fn public_examples() {
    assert_eq!(shard_occupancy_total(&[]), 0);
    assert_eq!(shard_occupancy_total(&[4]), 1);
}
