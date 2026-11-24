use task_146::*;

#[test]
fn edge_cases() {
    assert_eq!(saturating_debit_8(0, 2), 0);
    assert_eq!(saturating_debit_8(1, 3), 0);
    assert_eq!(saturating_debit_8(7, 4), 0);
    assert_eq!(saturating_debit_8(8, 5), 0);
    assert_eq!(saturating_debit_8(9, 6), 0);
    assert_eq!(saturating_debit_8(65535, 0), 65527);
    assert_eq!(saturating_debit_8(20, 1), 11);
    assert_eq!(saturating_debit_8(40, 2), 30);
    assert_eq!(saturating_debit_8(50, 3), 39);
    assert_eq!(saturating_debit_8(100, 4), 88);
    assert_eq!(saturating_debit_8(101, 5), 88);
    assert_eq!(saturating_debit_8(255, 6), 241);
    assert_eq!(saturating_debit_8(256, 0), 248);
    assert_eq!(saturating_debit_8(18, 3), 7);
    assert_eq!(saturating_debit_8(3, 18), 7);
    assert_eq!(saturating_debit_8(4, 4), 0);
    assert_eq!(saturating_debit_8(0, 0), 0);
    assert_eq!(saturating_debit_8(1, 0), 0);
    assert_eq!(saturating_debit_8(7, 0), 0);
    assert_eq!(saturating_debit_8(8, 0), 0);
    assert_eq!(saturating_debit_8(9, 0), 1);
    assert_eq!(saturating_debit_8(10, 0), 2);
    assert_eq!(saturating_debit_8(11, 0), 3);
    assert_eq!(saturating_debit_8(16, 0), 8);
    assert_eq!(saturating_debit_8(17, 0), 9);
    assert_eq!(saturating_debit_8(20, 0), 12);
    assert_eq!(saturating_debit_8(100, 0), 92);
}

#[test]
fn submission_marker() {
    assert_eq!(task_146::TRACK_PROOF, 0x52540000 | 146);
}
