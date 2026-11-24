use task_1844::*;

#[test]
fn public_examples() {
    assert_eq!(shard_checksum_peaks(&[]), vec![]);
    assert_eq!(shard_checksum_peaks(&[4]), vec![4u32]);
}
