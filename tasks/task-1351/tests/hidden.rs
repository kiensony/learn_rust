use task_1351::*;

#[test]
fn edge_cases() {
    assert_eq!(
        frame_lengths_trace(&[2, 7, 3, 1, 4, 0, 3, 9, 9, 2]),
        vec![2u32, 1u32, 0u32, 3u32]
    );
    assert_eq!(frame_lengths_trace(&[3, 1]), vec![]);
    assert_eq!(
        frame_lengths_trace(&[1, 0, 0, 1, 255]),
        vec![1u32, 0u32, 1u32]
    );
    assert_eq!(
        frame_lengths_trace(&[2, 3, 1, 2, 3, 1, 1, 2]),
        vec![2u32, 2u32, 1u32]
    );
    assert_eq!(frame_lengths_trace(&[255, 0]), vec![]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1351::TRACK_PROOF, 0x52540000 | 1351);
}
