use task_061::*;

#[test]
fn edge_cases() {
    assert_eq!(locker_number(0), 0);
    assert_eq!(locker_number(1), 1);
    assert_eq!(locker_number(255), 255);
}

#[test]
fn submission_marker() {
    assert_eq!(task_061::TRACK_PROOF, 0x52540000 | 61);
}
