use task_1821::*;

#[test]
fn edge_cases() {
    assert_eq!(shard_masks_trace(&[9, 2, 2, 8, 0, 5]), vec![11u32, 5u32]);
    assert_eq!(shard_masks_trace(&[0, 0, 0]), vec![0u32]);
    assert_eq!(shard_masks_trace(&[255, 1, 255, 2]), vec![255u32]);
    assert_eq!(
        shard_masks_trace(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![15u32, 31u32, 31u32]
    );
    assert_eq!(shard_masks_trace(&[17, 17, 1, 0, 2]), vec![17u32, 2u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1821::TRACK_PROOF, 0x52540000 | 1821);
}
