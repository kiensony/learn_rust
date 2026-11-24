use task_1843::*;

#[test]
fn public_examples() {
    assert_eq!(shard_checksum_bounds(&[]), None);
    assert_eq!(shard_checksum_bounds(&[4]), Some((4u32, 4u32)));
}
