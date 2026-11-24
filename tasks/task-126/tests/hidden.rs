use task_126::*;

#[test]
fn edge_cases() {
    assert_eq!(saturating_debit_3(0, 6), 3);
    assert_eq!(saturating_debit_3(1, 9), 7);
    assert_eq!(saturating_debit_3(2, 12), 11);
    assert_eq!(saturating_debit_3(3, 15), 15);
    assert_eq!(saturating_debit_3(4, 1), 2);
    assert_eq!(saturating_debit_3(65535, 4), 65532);
    assert_eq!(saturating_debit_3(20, 7), 24);
    assert_eq!(saturating_debit_3(40, 10), 47);
    assert_eq!(saturating_debit_3(50, 13), 60);
    assert_eq!(saturating_debit_3(100, 16), 113);
    assert_eq!(saturating_debit_3(101, 2), 100);
    assert_eq!(saturating_debit_3(255, 5), 257);
    assert_eq!(saturating_debit_3(256, 8), 261);
    assert_eq!(saturating_debit_3(65535, 1), 65532);
    assert_eq!(saturating_debit_3(0, 0), 0);
    assert_eq!(saturating_debit_3(1, 0), 0);
    assert_eq!(saturating_debit_3(2, 0), 0);
    assert_eq!(saturating_debit_3(3, 0), 0);
    assert_eq!(saturating_debit_3(5, 0), 2);
    assert_eq!(saturating_debit_3(6, 0), 3);
    assert_eq!(saturating_debit_3(7, 0), 4);
    assert_eq!(saturating_debit_3(20, 0), 17);
    assert_eq!(saturating_debit_3(100, 0), 97);
}

#[test]
fn submission_marker() {
    assert_eq!(task_126::TRACK_PROOF, 0x52540000 | 126);
}
