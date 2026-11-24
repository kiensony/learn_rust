use task_116::*;

#[test]
fn edge_cases() {
    assert_eq!(saturating_debit_2(0), 0);
    assert_eq!(saturating_debit_2(1), 0);
    assert_eq!(saturating_debit_2(1), 0);
    assert_eq!(saturating_debit_2(2), 0);
    assert_eq!(saturating_debit_2(3), 1);
    assert_eq!(saturating_debit_2(65535), 65533);
    assert_eq!(saturating_debit_2(20), 18);
    assert_eq!(saturating_debit_2(40), 38);
    assert_eq!(saturating_debit_2(50), 48);
    assert_eq!(saturating_debit_2(100), 98);
    assert_eq!(saturating_debit_2(101), 99);
    assert_eq!(saturating_debit_2(255), 253);
    assert_eq!(saturating_debit_2(256), 254);
}

#[test]
fn submission_marker() {
    assert_eq!(task_116::TRACK_PROOF, 0x52540000 | 116);
}
