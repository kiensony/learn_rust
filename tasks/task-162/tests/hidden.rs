use task_162::*;

#[test]
fn edge_cases() {
    assert_eq!(higher(7, 2), 7);
    assert_eq!(higher(-5, -1), -1);
    assert_eq!(higher(-32768, 32767), 32767);
}

#[test]
fn submission_marker() {
    assert_eq!(task_162::TRACK_PROOF, 0x52540000 | 162);
}
