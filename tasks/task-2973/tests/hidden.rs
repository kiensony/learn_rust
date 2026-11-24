use task_2973::*;

#[test]
fn edge_cases() {
    assert_eq!(queue_depth_bounds(&[9, 2, 2, 8, 0, 5]), Some((1u32, 4u32)));
    assert_eq!(queue_depth_bounds(&[0, 0, 0]), Some((0u32, 0u32)));
    assert_eq!(queue_depth_bounds(&[255, 1, 255, 2]), Some((1u32, 4u32)));
    assert_eq!(
        queue_depth_bounds(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        Some((1u32, 4u32))
    );
    assert_eq!(queue_depth_bounds(&[17, 17, 1, 0, 2]), Some((1u32, 3u32)));
}

#[test]
fn submission_marker() {
    assert_eq!(task_2973::TRACK_PROOF, 0x52540000 | 2973);
}
