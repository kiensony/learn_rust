use task_1842::*;

#[test]
fn public_examples() {
    assert_eq!(shard_checksum_total(&[]), 0);
    assert_eq!(shard_checksum_total(&[4]), 4);
}
