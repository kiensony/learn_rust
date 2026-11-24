use task_1367::*;

#[test]
fn edge_cases() {
    assert_eq!(
        frame_offsets_distinct(&[2, 7, 3, 1, 4, 0, 3, 9, 9, 2]),
        vec![0u32, 3u32, 5u32, 6u32]
    );
    assert_eq!(frame_offsets_distinct(&[3, 1]), vec![]);
    assert_eq!(
        frame_offsets_distinct(&[1, 0, 0, 1, 255]),
        vec![0u32, 2u32, 3u32]
    );
    assert_eq!(
        frame_offsets_distinct(&[2, 3, 1, 2, 3, 1, 1, 2]),
        vec![0u32, 3u32, 6u32]
    );
    assert_eq!(frame_offsets_distinct(&[255, 0]), vec![]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1367::TRACK_PROOF, 0x52540000 | 1367);
}
