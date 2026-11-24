use task_714::*;

#[test]
fn edge_cases() {
    assert_eq!(difference(None, Some(9)), None);
    assert_eq!(difference(Some(9), None), None);
    assert_eq!(difference(Some(-32768), Some(32767)), Some(65535));
}

#[test]
fn submission_marker() {
    assert_eq!(task_714::TRACK_PROOF, 0x52540000 | 714);
}
