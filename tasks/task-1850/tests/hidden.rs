use task_1850::*;

#[test]
fn edge_cases() {
    assert_eq!(shard_checksum_replay_mismatch(&[4], &[4]), None);
    assert_eq!(
        shard_checksum_replay_mismatch(&[9, 2, 2, 8, 0, 5], &[4, 777]),
        Some(1)
    );
    assert_eq!(shard_checksum_replay_mismatch(&[0, 0, 0], &[]), Some(0));
    assert_eq!(
        shard_checksum_replay_mismatch(&[255, 1, 255, 2], &[9999]),
        Some(0)
    );
    assert_eq!(
        shard_checksum_replay_mismatch(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], &[13, 26]),
        Some(0)
    );
    assert_eq!(
        shard_checksum_replay_mismatch(&[17, 17, 1, 0, 2], &[3]),
        None
    );
    assert_eq!(
        shard_checksum_replay_mismatch(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], &[26, 13, 1]),
        Some(2)
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1850::TRACK_PROOF, 0x52540000 | 1850);
}
