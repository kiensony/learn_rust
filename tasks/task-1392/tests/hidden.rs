use task_1392::*;

#[test]
fn edge_cases() {
    assert_eq!(frame_maxima_total(&[2, 7, 3, 1, 4, 0, 3, 9, 9, 2]), 20);
    assert_eq!(frame_maxima_total(&[3, 1]), 0);
    assert_eq!(frame_maxima_total(&[1, 0, 0, 1, 255]), 255);
    assert_eq!(frame_maxima_total(&[2, 3, 1, 2, 3, 1, 1, 2]), 8);
    assert_eq!(frame_maxima_total(&[255, 0]), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1392::TRACK_PROOF, 0x52540000 | 1392);
}
