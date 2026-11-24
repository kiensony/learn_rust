use task_213::*;

#[test]
fn edge_cases() {
    assert_eq!(running_deposits_2(0), 0);
    assert_eq!(running_deposits_2(1), 2);
    assert_eq!(running_deposits_2(1), 2);
    assert_eq!(running_deposits_2(2), 6);
    assert_eq!(running_deposits_2(3), 12);
    assert_eq!(running_deposits_2(255), 65280);
}

#[test]
fn submission_marker() {
    assert_eq!(task_213::TRACK_PROOF, 0x52540000 | 213);
}
