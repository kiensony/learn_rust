use task_1842::*;

#[test]
fn edge_cases() {
    assert_eq!(shard_checksum_total(&[9, 2, 2, 8, 0, 5]), 4);
    assert_eq!(shard_checksum_total(&[0, 0, 0]), 0);
    assert_eq!(shard_checksum_total(&[255, 1, 255, 2]), 3);
    assert_eq!(
        shard_checksum_total(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        39
    );
    assert_eq!(shard_checksum_total(&[17, 17, 1, 0, 2]), 3);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1842::TRACK_PROOF, 0x52540000 | 1842);
}
