use task_1395::*;

#[test]
fn edge_cases() {
    assert_eq!(
        frame_maxima_changes(&[2, 7, 3, 1, 4, 0, 3, 9, 9, 2]),
        vec![7u32, 4u32, 0u32, 9u32]
    );
    assert_eq!(frame_maxima_changes(&[3, 1]), vec![]);
    assert_eq!(frame_maxima_changes(&[1, 0, 0, 1, 255]), vec![0u32, 255u32]);
    assert_eq!(
        frame_maxima_changes(&[2, 3, 1, 2, 3, 1, 1, 2]),
        vec![3u32, 2u32]
    );
    assert_eq!(frame_maxima_changes(&[255, 0]), vec![]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1395::TRACK_PROOF, 0x52540000 | 1395);
}
