use task_1850::*;

#[test]
fn public_examples() {
    assert_eq!(
        shard_checksum_replay_mismatch(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], &[26]),
        Some(1)
    );
    assert_eq!(shard_checksum_replay_mismatch(&[], &[]), None);
}
