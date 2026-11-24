use task_1836::*;

#[test]
fn public_examples() {
    assert_eq!(shard_occupancy_descending_steps(&[]), 0);
    assert_eq!(shard_occupancy_descending_steps(&[4]), 0);
}
