use task_120::*;

#[test]
fn edge_cases() {
    assert_eq!(fractional_fee_2(13), 0);
    assert_eq!(fractional_fee_2(0), 0);
    assert_eq!(fractional_fee_2(1), 0);
    assert_eq!(fractional_fee_2(1), 0);
    assert_eq!(fractional_fee_2(2), 0);
    assert_eq!(fractional_fee_2(3), 0);
    assert_eq!(fractional_fee_2(20), 0);
    assert_eq!(fractional_fee_2(40), 0);
    assert_eq!(fractional_fee_2(50), 1);
    assert_eq!(fractional_fee_2(100), 2);
    assert_eq!(fractional_fee_2(101), 2);
    assert_eq!(fractional_fee_2(255), 5);
    assert_eq!(fractional_fee_2(256), 5);
}

#[test]
fn submission_marker() {
    assert_eq!(task_120::TRACK_PROOF, 0x52540000 | 120);
}
