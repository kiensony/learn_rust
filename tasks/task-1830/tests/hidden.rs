use task_1830::*;

#[test]
fn edge_cases() {
    assert_eq!(shard_masks_reverse(&[9, 2, 2, 8, 0, 5]), vec![5u32, 11u32]);
    assert_eq!(shard_masks_reverse(&[0, 0, 0]), vec![0u32]);
    assert_eq!(shard_masks_reverse(&[255, 1, 255, 2]), vec![255u32]);
    assert_eq!(
        shard_masks_reverse(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![31u32, 31u32, 15u32]
    );
    assert_eq!(shard_masks_reverse(&[17, 17, 1, 0, 2]), vec![2u32, 17u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1830::TRACK_PROOF, 0x52540000 | 1830);
}
