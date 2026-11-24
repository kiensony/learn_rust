use task_188::*;

#[test]
fn edge_cases() {
    assert_eq!(priority_fee_5(0, 2), 0);
    assert_eq!(priority_fee_5(1, 0), 5);
    assert_eq!(priority_fee_5(4, 1), 5);
    assert_eq!(priority_fee_5(5, 2), 5);
    assert_eq!(priority_fee_5(6, 0), 5);
    assert_eq!(priority_fee_5(65535, 1), 10);
    assert_eq!(priority_fee_5(7, 2), 5);
    assert_eq!(priority_fee_5(8, 0), 10);
    assert_eq!(priority_fee_5(10, 1), 10);
    assert_eq!(priority_fee_5(11, 2), 10);
    assert_eq!(priority_fee_5(0, 9), 0);
    assert_eq!(priority_fee_5(100, 0), 10);
    assert_eq!(priority_fee_5(100, 1), 10);
    assert_eq!(priority_fee_5(0, 0), 0);
    assert_eq!(priority_fee_5(5, 0), 5);
    assert_eq!(priority_fee_5(7, 0), 5);
    assert_eq!(priority_fee_5(10, 0), 10);
    assert_eq!(priority_fee_5(11, 0), 10);
    assert_eq!(priority_fee_5(20, 0), 10);
}

#[test]
fn submission_marker() {
    assert_eq!(task_188::TRACK_PROOF, 0x52540000 | 188);
}
