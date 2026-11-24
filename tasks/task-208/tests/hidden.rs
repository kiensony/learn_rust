use task_208::*;

#[test]
fn edge_cases() {
    assert_eq!(priority_fee_10(65535, 0), 0);
    assert_eq!(priority_fee_10(1, 2), 10);
    assert_eq!(priority_fee_10(65535, 9), 10);
    assert_eq!(priority_fee_10(10, 11), 10);
    assert_eq!(priority_fee_10(65535, 11), 10);
    assert_eq!(priority_fee_10(65535, 65535), 20);
    assert_eq!(priority_fee_10(65535, 12), 10);
    assert_eq!(priority_fee_10(65535, 20), 20);
    assert_eq!(priority_fee_10(21, 22), 20);
    assert_eq!(priority_fee_10(1, 0), 0);
    assert_eq!(priority_fee_10(0, 1), 0);
    assert_eq!(priority_fee_10(0, 0), 0);
    assert_eq!(priority_fee_10(1, 1), 10);
    assert_eq!(priority_fee_10(9, 9), 10);
    assert_eq!(priority_fee_10(10, 10), 10);
    assert_eq!(priority_fee_10(11, 11), 10);
    assert_eq!(priority_fee_10(12, 12), 10);
    assert_eq!(priority_fee_10(13, 13), 20);
    assert_eq!(priority_fee_10(20, 20), 20);
    assert_eq!(priority_fee_10(21, 21), 20);
}

#[test]
fn submission_marker() {
    assert_eq!(task_208::TRACK_PROOF, 0x52540000 | 208);
}
