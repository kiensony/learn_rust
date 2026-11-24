use task_468::*;

#[test]
fn edge_cases() {
    assert_eq!(shift([0, 0, 0]), [0, 0, 0]);
    assert_eq!(shift([255, 1, 0]), [1, 0, 255]);
    assert_eq!(shift([1, 1, 2]), [1, 2, 1]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_468::TRACK_PROOF, 0x52540000 | 468);
}
