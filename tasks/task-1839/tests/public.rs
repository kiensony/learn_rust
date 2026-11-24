use task_1839::*;

#[test]
fn public_examples() {
    assert_eq!(shard_occupancy_longest_run(&[]), 0);
    assert_eq!(shard_occupancy_longest_run(&[4]), 1);
}
