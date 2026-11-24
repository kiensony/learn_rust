use task_616::*;

#[test]
fn edge_cases() {
    assert_eq!(after(0, &[]), 0);
    assert_eq!(after(250, &[9]), 255);
    assert_eq!(after(255, &[1, 1]), 255);
}

#[test]
fn submission_marker() {
    assert_eq!(task_616::TRACK_PROOF, 0x52540000 | 616);
}
