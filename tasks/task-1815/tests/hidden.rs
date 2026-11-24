use task_1815::*;

#[test]
fn edge_cases() {
    assert_eq!(shard_maxima_changes(&[9, 2, 2, 8, 0, 5]), vec![9u32, 8u32]);
    assert_eq!(shard_maxima_changes(&[0, 0, 0]), vec![0u32]);
    assert_eq!(shard_maxima_changes(&[255, 1, 255, 2]), vec![255u32, 2u32]);
    assert_eq!(
        shard_maxima_changes(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![9u32, 21u32, 26u32, 31u32]
    );
    assert_eq!(shard_maxima_changes(&[17, 17, 1, 0, 2]), vec![17u32, 2u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1815::TRACK_PROOF, 0x52540000 | 1815);
}
