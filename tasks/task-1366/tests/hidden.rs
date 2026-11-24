use task_1366::*;

#[test]
fn edge_cases() {
    assert_eq!(
        frame_offsets_descending_steps(&[2, 7, 3, 1, 4, 0, 3, 9, 9, 2]),
        0
    );
    assert_eq!(frame_offsets_descending_steps(&[3, 1]), 0);
    assert_eq!(frame_offsets_descending_steps(&[1, 0, 0, 1, 255]), 0);
    assert_eq!(frame_offsets_descending_steps(&[2, 3, 1, 2, 3, 1, 1, 2]), 0);
    assert_eq!(frame_offsets_descending_steps(&[255, 0]), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1366::TRACK_PROOF, 0x52540000 | 1366);
}
