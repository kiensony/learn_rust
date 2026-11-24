use task_1838::*;

#[test]
fn public_examples() {
    assert_eq!(shard_occupancy_steps(&[]), vec![]);
    assert_eq!(shard_occupancy_steps(&[4]), vec![]);
}
