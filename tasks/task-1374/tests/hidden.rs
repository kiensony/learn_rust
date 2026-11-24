use task_1374::*;

#[test]
fn edge_cases() {
    assert_eq!(
        frame_checksum_peaks(&[2, 7, 3, 1, 4, 0, 3, 9, 9, 2]),
        vec![4u32]
    );
    assert_eq!(frame_checksum_peaks(&[3, 1]), vec![]);
    assert_eq!(frame_checksum_peaks(&[1, 0, 0, 1, 255]), vec![0u32, 255u32]);
    assert_eq!(frame_checksum_peaks(&[2, 3, 1, 2, 3, 1, 1, 2]), vec![2u32]);
    assert_eq!(frame_checksum_peaks(&[255, 0]), vec![]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1374::TRACK_PROOF, 0x52540000 | 1374);
}
