use task_1393::*;

#[test]
fn edge_cases() {
    assert_eq!(
        frame_maxima_bounds(&[2, 7, 3, 1, 4, 0, 3, 9, 9, 2]),
        Some((0u32, 9u32))
    );
    assert_eq!(frame_maxima_bounds(&[3, 1]), None);
    assert_eq!(
        frame_maxima_bounds(&[1, 0, 0, 1, 255]),
        Some((0u32, 255u32))
    );
    assert_eq!(
        frame_maxima_bounds(&[2, 3, 1, 2, 3, 1, 1, 2]),
        Some((2u32, 3u32))
    );
    assert_eq!(frame_maxima_bounds(&[255, 0]), None);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1393::TRACK_PROOF, 0x52540000 | 1393);
}
