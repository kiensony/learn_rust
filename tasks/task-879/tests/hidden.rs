use task_879::*;

#[test]
fn edge_cases() {
    assert_eq!(code_size(2), 4);
    assert_eq!(code_size(6), 1);
    assert_eq!(code_size(255), 1);
}

#[test]
fn submission_marker() {
    assert_eq!(task_879::TRACK_PROOF, 0x52540000 | 879);
}
