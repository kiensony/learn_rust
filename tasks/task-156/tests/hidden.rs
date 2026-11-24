use task_156::*;

#[test]
fn edge_cases() {
    assert_eq!(saturating_debit_10(65535, 0), 0);
    assert_eq!(saturating_debit_10(1, 2), 0);
    assert_eq!(saturating_debit_10(65535, 9), 0);
    assert_eq!(saturating_debit_10(10, 11), 0);
    assert_eq!(saturating_debit_10(65535, 11), 1);
    assert_eq!(saturating_debit_10(65535, 65535), 65525);
    assert_eq!(saturating_debit_10(65535, 20), 10);
    assert_eq!(saturating_debit_10(40, 41), 30);
    assert_eq!(saturating_debit_10(65535, 50), 40);
    assert_eq!(saturating_debit_10(100, 101), 90);
    assert_eq!(saturating_debit_10(65535, 101), 91);
    assert_eq!(saturating_debit_10(255, 256), 245);
    assert_eq!(saturating_debit_10(65535, 256), 246);
    assert_eq!(saturating_debit_10(1, 0), 0);
    assert_eq!(saturating_debit_10(0, 1), 0);
    assert_eq!(saturating_debit_10(0, 0), 0);
    assert_eq!(saturating_debit_10(1, 1), 0);
    assert_eq!(saturating_debit_10(9, 9), 0);
    assert_eq!(saturating_debit_10(10, 10), 0);
    assert_eq!(saturating_debit_10(11, 11), 1);
    assert_eq!(saturating_debit_10(12, 12), 2);
    assert_eq!(saturating_debit_10(13, 13), 3);
    assert_eq!(saturating_debit_10(20, 20), 10);
    assert_eq!(saturating_debit_10(21, 21), 11);
}

#[test]
fn submission_marker() {
    assert_eq!(task_156::TRACK_PROOF, 0x52540000 | 156);
}
