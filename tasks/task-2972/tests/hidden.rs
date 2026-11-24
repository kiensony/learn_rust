use task_2972::*;

#[test]
fn edge_cases() {
    assert_eq!(queue_depth_total(&[9, 2, 2, 8, 0, 5]), 17);
    assert_eq!(queue_depth_total(&[0, 0, 0]), 0);
    assert_eq!(queue_depth_total(&[255, 1, 255, 2]), 10);
    assert_eq!(queue_depth_total(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]), 33);
    assert_eq!(queue_depth_total(&[17, 17, 1, 0, 2]), 11);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2972::TRACK_PROOF, 0x52540000 | 2972);
}
