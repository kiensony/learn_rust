use task_1780::*;

#[test]
fn edge_cases() {
    assert_eq!(
        build_mask_reverse(&[9, 2, 2, 8, 0, 5]),
        vec![5u32, 0u32, 8u32, 2u32, 2u32, 9u32]
    );
    assert_eq!(build_mask_reverse(&[0, 0, 0]), vec![0u32, 0u32, 0u32]);
    assert_eq!(
        build_mask_reverse(&[255, 1, 255, 2]),
        vec![2u32, 63u32, 1u32, 63u32]
    );
    assert_eq!(
        build_mask_reverse(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![
            31u32, 8u32, 26u32, 0u32, 15u32, 21u32, 12u32, 9u32, 6u32, 3u32
        ]
    );
    assert_eq!(
        build_mask_reverse(&[17, 17, 1, 0, 2]),
        vec![2u32, 0u32, 1u32, 17u32, 17u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1780::TRACK_PROOF, 0x52540000 | 1780);
}
