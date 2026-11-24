use task_1380::*;

#[test]
fn edge_cases() {
    assert_eq!(
        frame_checksum_reverse(&[2, 7, 3, 1, 4, 0, 3, 9, 9, 2]),
        vec![2u32, 0u32, 4u32, 4u32]
    );
    assert_eq!(frame_checksum_reverse(&[3, 1]), vec![]);
    assert_eq!(
        frame_checksum_reverse(&[1, 0, 0, 1, 255]),
        vec![255u32, 0u32, 0u32]
    );
    assert_eq!(
        frame_checksum_reverse(&[2, 3, 1, 2, 3, 1, 1, 2]),
        vec![2u32, 2u32, 2u32]
    );
    assert_eq!(frame_checksum_reverse(&[255, 0]), vec![]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1380::TRACK_PROOF, 0x52540000 | 1380);
}
