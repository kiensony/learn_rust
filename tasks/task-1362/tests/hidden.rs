use task_1362::*;

#[test]
fn edge_cases() {
    assert_eq!(frame_offsets_total(&[2, 7, 3, 1, 4, 0, 3, 9, 9, 2]), 14);
    assert_eq!(frame_offsets_total(&[3, 1]), 0);
    assert_eq!(frame_offsets_total(&[1, 0, 0, 1, 255]), 5);
    assert_eq!(frame_offsets_total(&[2, 3, 1, 2, 3, 1, 1, 2]), 9);
    assert_eq!(frame_offsets_total(&[255, 0]), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1362::TRACK_PROOF, 0x52540000 | 1362);
}
