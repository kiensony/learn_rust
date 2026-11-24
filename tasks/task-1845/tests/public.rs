use task_1845::*;

#[test]
fn public_examples() {
    assert_eq!(shard_checksum_changes(&[]), vec![]);
    assert_eq!(shard_checksum_changes(&[4]), vec![4u32]);
}
