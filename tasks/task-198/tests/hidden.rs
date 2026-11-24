use task_198::*;

#[test]
fn edge_cases() {
    assert_eq!(priority_fee_8(0, 2), 8);
    assert_eq!(priority_fee_8(1, 3), 8);
    assert_eq!(priority_fee_8(7, 4), 8);
    assert_eq!(priority_fee_8(8, 5), 8);
    assert_eq!(priority_fee_8(9, 6), 8);
    assert_eq!(priority_fee_8(65535, 0), 16);
    assert_eq!(priority_fee_8(10, 1), 8);
    assert_eq!(priority_fee_8(11, 2), 8);
    assert_eq!(priority_fee_8(16, 3), 16);
    assert_eq!(priority_fee_8(17, 4), 16);
    assert_eq!(priority_fee_8(18, 3), 16);
    assert_eq!(priority_fee_8(3, 18), 16);
    assert_eq!(priority_fee_8(4, 4), 0);
    assert_eq!(priority_fee_8(0, 0), 0);
    assert_eq!(priority_fee_8(1, 0), 8);
    assert_eq!(priority_fee_8(7, 0), 8);
    assert_eq!(priority_fee_8(8, 0), 8);
    assert_eq!(priority_fee_8(9, 0), 8);
    assert_eq!(priority_fee_8(10, 0), 8);
    assert_eq!(priority_fee_8(11, 0), 16);
    assert_eq!(priority_fee_8(16, 0), 16);
    assert_eq!(priority_fee_8(17, 0), 16);
    assert_eq!(priority_fee_8(20, 0), 16);
    assert_eq!(priority_fee_8(100, 0), 16);
}

#[test]
fn submission_marker() {
    assert_eq!(task_198::TRACK_PROOF, 0x52540000 | 198);
}
