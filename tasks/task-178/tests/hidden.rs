use task_178::*;

#[test]
fn edge_cases() {
    assert_eq!(priority_fee_3(0, 6), 6);
    assert_eq!(priority_fee_3(1, 9), 6);
    assert_eq!(priority_fee_3(2, 12), 6);
    assert_eq!(priority_fee_3(3, 15), 6);
    assert_eq!(priority_fee_3(4, 1), 3);
    assert_eq!(priority_fee_3(65535, 4), 6);
    assert_eq!(priority_fee_3(5, 7), 6);
    assert_eq!(priority_fee_3(6, 10), 6);
    assert_eq!(priority_fee_3(6, 13), 6);
    assert_eq!(priority_fee_3(7, 16), 6);
    assert_eq!(priority_fee_3(65535, 1), 6);
    assert_eq!(priority_fee_3(0, 0), 0);
    assert_eq!(priority_fee_3(1, 0), 3);
    assert_eq!(priority_fee_3(2, 0), 3);
    assert_eq!(priority_fee_3(3, 0), 3);
    assert_eq!(priority_fee_3(5, 0), 3);
    assert_eq!(priority_fee_3(6, 0), 6);
    assert_eq!(priority_fee_3(7, 0), 6);
    assert_eq!(priority_fee_3(20, 0), 6);
    assert_eq!(priority_fee_3(100, 0), 6);
}

#[test]
fn submission_marker() {
    assert_eq!(task_178::TRACK_PROOF, 0x52540000 | 178);
}
