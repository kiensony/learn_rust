use task_269::*;

#[test]
fn edge_cases() {
    assert_eq!(bump(0), 1);
    assert_eq!(bump(254), 255);
    assert_eq!(bump(255), 255);
}

#[test]
fn submission_marker() {
    assert_eq!(task_269::TRACK_PROOF, 0x52540000 | 269);
}
