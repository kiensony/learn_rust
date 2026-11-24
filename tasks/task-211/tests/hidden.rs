use task_211::*;

#[test]
fn edge_cases() {
    assert_eq!(countdown_total(0), 0);
    assert_eq!(countdown_total(1), 1);
    assert_eq!(countdown_total(255), 32640);
}

#[test]
fn submission_marker() {
    assert_eq!(task_211::TRACK_PROOF, 0x52540000 | 211);
}
