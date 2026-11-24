use task_130::*;

#[test]
fn edge_cases() {
    assert_eq!(fractional_fee_3(13, 3), 0);
    assert_eq!(fractional_fee_3(0, 6), 0);
    assert_eq!(fractional_fee_3(1, 9), 0);
    assert_eq!(fractional_fee_3(2, 12), 0);
    assert_eq!(fractional_fee_3(3, 15), 0);
    assert_eq!(fractional_fee_3(4, 1), 0);
    assert_eq!(fractional_fee_3(20, 7), 0);
    assert_eq!(fractional_fee_3(40, 10), 1);
    assert_eq!(fractional_fee_3(50, 13), 1);
    assert_eq!(fractional_fee_3(100, 16), 3);
    assert_eq!(fractional_fee_3(101, 2), 3);
    assert_eq!(fractional_fee_3(255, 5), 7);
    assert_eq!(fractional_fee_3(256, 8), 7);
    assert_eq!(fractional_fee_3(65535, 1), 1966);
    assert_eq!(fractional_fee_3(0, 0), 0);
    assert_eq!(fractional_fee_3(1, 0), 0);
    assert_eq!(fractional_fee_3(2, 0), 0);
    assert_eq!(fractional_fee_3(3, 0), 0);
    assert_eq!(fractional_fee_3(5, 0), 0);
    assert_eq!(fractional_fee_3(6, 0), 0);
    assert_eq!(fractional_fee_3(7, 0), 0);
    assert_eq!(fractional_fee_3(20, 0), 0);
    assert_eq!(fractional_fee_3(100, 0), 3);
}

#[test]
fn submission_marker() {
    assert_eq!(task_130::TRACK_PROOF, 0x52540000 | 130);
}
