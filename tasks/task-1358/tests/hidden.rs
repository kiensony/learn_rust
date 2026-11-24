use task_1358::*;

#[test]
fn edge_cases() {
    assert_eq!(
        frame_lengths_steps(&[2, 7, 3, 1, 4, 0, 3, 9, 9, 2]),
        vec![1u32, 1u32, 3u32]
    );
    assert_eq!(frame_lengths_steps(&[3, 1]), vec![]);
    assert_eq!(frame_lengths_steps(&[1, 0, 0, 1, 255]), vec![1u32, 1u32]);
    assert_eq!(
        frame_lengths_steps(&[2, 3, 1, 2, 3, 1, 1, 2]),
        vec![0u32, 1u32]
    );
    assert_eq!(frame_lengths_steps(&[255, 0]), vec![]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1358::TRACK_PROOF, 0x52540000 | 1358);
}
