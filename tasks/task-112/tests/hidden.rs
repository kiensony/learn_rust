use task_112::*;

#[test]
fn edge_cases() {
    assert_eq!(degrees(0), 0.0);
    assert_eq!(degrees(4), 1.0);
    assert_eq!(degrees(-32768), -8192.0);
    assert_eq!(degrees(32767), 8191.75);
}

#[test]
fn submission_marker() {
    assert_eq!(task_112::TRACK_PROOF, 0x52540000 | 112);
}
