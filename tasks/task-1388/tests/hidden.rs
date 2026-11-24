use task_1388::*;

#[test]
fn edge_cases() {
    assert_eq!(
        first_frame_byte_steps(&[2, 7, 3, 1, 4, 0, 3, 9, 9, 2]),
        vec![3u32, 4u32, 9u32]
    );
    assert_eq!(first_frame_byte_steps(&[3, 1]), vec![]);
    assert_eq!(
        first_frame_byte_steps(&[1, 0, 0, 1, 255]),
        vec![0u32, 255u32]
    );
    assert_eq!(
        first_frame_byte_steps(&[2, 3, 1, 2, 3, 1, 1, 2]),
        vec![0u32, 1u32]
    );
    assert_eq!(first_frame_byte_steps(&[255, 0]), vec![]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1388::TRACK_PROOF, 0x52540000 | 1388);
}
