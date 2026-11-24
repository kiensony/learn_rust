use task_1833::*;

#[test]
fn public_examples() {
    assert_eq!(shard_occupancy_bounds(&[]), None);
    assert_eq!(shard_occupancy_bounds(&[4]), Some((1u32, 1u32)));
}
