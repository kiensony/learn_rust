use task_1370::*;

#[test]
fn edge_cases() {
    assert_eq!(
        frame_offsets_reverse(&[2, 7, 3, 1, 4, 0, 3, 9, 9, 2]),
        vec![6u32, 5u32, 3u32, 0u32]
    );
    assert_eq!(frame_offsets_reverse(&[3, 1]), vec![]);
    assert_eq!(
        frame_offsets_reverse(&[1, 0, 0, 1, 255]),
        vec![3u32, 2u32, 0u32]
    );
    assert_eq!(
        frame_offsets_reverse(&[2, 3, 1, 2, 3, 1, 1, 2]),
        vec![6u32, 3u32, 0u32]
    );
    assert_eq!(frame_offsets_reverse(&[255, 0]), vec![]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1370::TRACK_PROOF, 0x52540000 | 1370);
}
