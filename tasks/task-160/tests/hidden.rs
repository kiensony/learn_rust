use task_160::*;

#[test]
fn edge_cases() {
    assert_eq!(fractional_fee_10(65535, 0), 0);
    assert_eq!(fractional_fee_10(1, 2), 0);
    assert_eq!(fractional_fee_10(65535, 9), 0);
    assert_eq!(fractional_fee_10(10, 11), 1);
    assert_eq!(fractional_fee_10(65535, 11), 1);
    assert_eq!(fractional_fee_10(65535, 65535), 6553);
    assert_eq!(fractional_fee_10(65535, 20), 2);
    assert_eq!(fractional_fee_10(40, 41), 4);
    assert_eq!(fractional_fee_10(65535, 50), 5);
    assert_eq!(fractional_fee_10(100, 101), 10);
    assert_eq!(fractional_fee_10(65535, 101), 10);
    assert_eq!(fractional_fee_10(255, 256), 25);
    assert_eq!(fractional_fee_10(65535, 256), 25);
    assert_eq!(fractional_fee_10(1, 0), 0);
    assert_eq!(fractional_fee_10(0, 1), 0);
    assert_eq!(fractional_fee_10(0, 0), 0);
    assert_eq!(fractional_fee_10(1, 1), 0);
    assert_eq!(fractional_fee_10(9, 9), 0);
    assert_eq!(fractional_fee_10(10, 10), 1);
    assert_eq!(fractional_fee_10(11, 11), 1);
    assert_eq!(fractional_fee_10(12, 12), 1);
    assert_eq!(fractional_fee_10(13, 13), 1);
    assert_eq!(fractional_fee_10(20, 20), 2);
    assert_eq!(fractional_fee_10(21, 21), 2);
}

#[test]
fn submission_marker() {
    assert_eq!(task_160::TRACK_PROOF, 0x52540000 | 160);
}
