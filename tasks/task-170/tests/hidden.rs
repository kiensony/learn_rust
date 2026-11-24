use task_170::*;

#[test]
fn edge_cases() {
    assert_eq!(instruction('r'), "wait");
    assert_eq!(instruction('?'), "wait");
    assert_eq!(instruction('雪'), "wait");
}

#[test]
fn submission_marker() {
    assert_eq!(task_170::TRACK_PROOF, 0x52540000 | 170);
}
