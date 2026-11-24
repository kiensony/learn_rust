use task_1843::*;

#[test]
fn edge_cases() {
    assert_eq!(
        shard_checksum_bounds(&[9, 2, 2, 8, 0, 5]),
        Some((4u32, 4u32))
    );
    assert_eq!(shard_checksum_bounds(&[0, 0, 0]), Some((0u32, 0u32)));
    assert_eq!(shard_checksum_bounds(&[255, 1, 255, 2]), Some((3u32, 3u32)));
    assert_eq!(
        shard_checksum_bounds(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        Some((13u32, 26u32))
    );
    assert_eq!(
        shard_checksum_bounds(&[17, 17, 1, 0, 2]),
        Some((3u32, 3u32))
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1843::TRACK_PROOF, 0x52540000 | 1843);
}
