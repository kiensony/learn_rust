use task_1398::*;

#[test]
fn edge_cases() {
    assert_eq!(frame_checksum_rolling_load(&[0], 1), vec![0u64]);
    assert_eq!(
        frame_checksum_rolling_load(&[2, 7, 3, 1, 4, 0, 3, 9, 9, 2], 2),
        vec![8u64, 4u64, 2u64]
    );
    assert_eq!(frame_checksum_rolling_load(&[3, 1], 0), vec![]);
    assert_eq!(frame_checksum_rolling_load(&[1, 0, 0, 1, 255], 100), vec![]);
    assert_eq!(
        frame_checksum_rolling_load(&[2, 3, 1, 2, 3, 1, 1, 2], 3),
        vec![6u64]
    );
    assert_eq!(frame_checksum_rolling_load(&[255, 0], 1), vec![]);
    assert_eq!(
        frame_checksum_rolling_load(&[2, 7, 3, 1, 4, 0, 3, 9, 9, 2], 2),
        vec![8u64, 4u64, 2u64]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1398::TRACK_PROOF, 0x52540000 | 1398);
}
