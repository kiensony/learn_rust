use task_1353::*;

#[test]
fn edge_cases() {
    assert_eq!(
        frame_lengths_bounds(&[2, 7, 3, 1, 4, 0, 3, 9, 9, 2]),
        Some((0u32, 3u32))
    );
    assert_eq!(frame_lengths_bounds(&[3, 1]), None);
    assert_eq!(frame_lengths_bounds(&[1, 0, 0, 1, 255]), Some((0u32, 1u32)));
    assert_eq!(
        frame_lengths_bounds(&[2, 3, 1, 2, 3, 1, 1, 2]),
        Some((1u32, 2u32))
    );
    assert_eq!(frame_lengths_bounds(&[255, 0]), None);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1353::TRACK_PROOF, 0x52540000 | 1353);
}
