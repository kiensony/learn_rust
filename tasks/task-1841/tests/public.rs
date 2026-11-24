use task_1841::*;

#[test]
fn public_examples() {
    assert_eq!(shard_checksum_trace(&[]), vec![]);
    assert_eq!(shard_checksum_trace(&[4]), vec![4u32]);
}
