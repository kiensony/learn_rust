use task_150::*;

#[test]
fn edge_cases() {
    assert_eq!(fractional_fee_8(13, 1), 0);
    assert_eq!(fractional_fee_8(0, 2), 0);
    assert_eq!(fractional_fee_8(1, 3), 0);
    assert_eq!(fractional_fee_8(7, 4), 0);
    assert_eq!(fractional_fee_8(8, 5), 0);
    assert_eq!(fractional_fee_8(9, 6), 0);
    assert_eq!(fractional_fee_8(20, 1), 1);
    assert_eq!(fractional_fee_8(40, 2), 3);
    assert_eq!(fractional_fee_8(50, 3), 3);
    assert_eq!(fractional_fee_8(100, 4), 7);
    assert_eq!(fractional_fee_8(101, 5), 7);
    assert_eq!(fractional_fee_8(255, 6), 19);
    assert_eq!(fractional_fee_8(256, 0), 20);
    assert_eq!(fractional_fee_8(18, 3), 1);
    assert_eq!(fractional_fee_8(3, 18), 1);
    assert_eq!(fractional_fee_8(4, 4), 0);
    assert_eq!(fractional_fee_8(0, 0), 0);
    assert_eq!(fractional_fee_8(1, 0), 0);
    assert_eq!(fractional_fee_8(7, 0), 0);
    assert_eq!(fractional_fee_8(8, 0), 0);
    assert_eq!(fractional_fee_8(9, 0), 0);
    assert_eq!(fractional_fee_8(10, 0), 0);
    assert_eq!(fractional_fee_8(11, 0), 0);
    assert_eq!(fractional_fee_8(16, 0), 1);
    assert_eq!(fractional_fee_8(17, 0), 1);
    assert_eq!(fractional_fee_8(20, 0), 1);
    assert_eq!(fractional_fee_8(100, 0), 8);
}

#[test]
fn submission_marker() {
    assert_eq!(task_150::TRACK_PROOF, 0x52540000 | 150);
}
