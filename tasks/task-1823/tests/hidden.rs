use task_1823::*;

#[test]
fn edge_cases() {
    assert_eq!(shard_masks_bounds(&[9, 2, 2, 8, 0, 5]), Some((5u32, 11u32)));
    assert_eq!(shard_masks_bounds(&[0, 0, 0]), Some((0u32, 0u32)));
    assert_eq!(
        shard_masks_bounds(&[255, 1, 255, 2]),
        Some((255u32, 255u32))
    );
    assert_eq!(
        shard_masks_bounds(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        Some((15u32, 31u32))
    );
    assert_eq!(shard_masks_bounds(&[17, 17, 1, 0, 2]), Some((2u32, 17u32)));
}

#[test]
fn submission_marker() {
    assert_eq!(task_1823::TRACK_PROOF, 0x52540000 | 1823);
}
