use task_140::*;

#[test]
fn edge_cases() {
    assert_eq!(fractional_fee_5(13, 1), 0);
    assert_eq!(fractional_fee_5(0, 2), 0);
    assert_eq!(fractional_fee_5(1, 0), 0);
    assert_eq!(fractional_fee_5(4, 1), 0);
    assert_eq!(fractional_fee_5(5, 2), 0);
    assert_eq!(fractional_fee_5(6, 0), 0);
    assert_eq!(fractional_fee_5(20, 2), 0);
    assert_eq!(fractional_fee_5(40, 0), 2);
    assert_eq!(fractional_fee_5(50, 1), 2);
    assert_eq!(fractional_fee_5(100, 2), 4);
    assert_eq!(fractional_fee_5(101, 0), 5);
    assert_eq!(fractional_fee_5(255, 1), 12);
    assert_eq!(fractional_fee_5(256, 2), 12);
    assert_eq!(fractional_fee_5(0, 9), 0);
    assert_eq!(fractional_fee_5(100, 0), 5);
    assert_eq!(fractional_fee_5(100, 1), 4);
    assert_eq!(fractional_fee_5(0, 0), 0);
    assert_eq!(fractional_fee_5(5, 0), 0);
    assert_eq!(fractional_fee_5(7, 0), 0);
    assert_eq!(fractional_fee_5(8, 0), 0);
    assert_eq!(fractional_fee_5(10, 0), 0);
    assert_eq!(fractional_fee_5(11, 0), 0);
    assert_eq!(fractional_fee_5(20, 0), 1);
}

#[test]
fn submission_marker() {
    assert_eq!(task_140::TRACK_PROOF, 0x52540000 | 140);
}
