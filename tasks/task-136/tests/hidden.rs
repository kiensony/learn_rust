use task_136::*;

#[test]
fn edge_cases() {
    assert_eq!(saturating_debit_5(0, 2), 0);
    assert_eq!(saturating_debit_5(1, 0), 0);
    assert_eq!(saturating_debit_5(4, 1), 0);
    assert_eq!(saturating_debit_5(5, 2), 0);
    assert_eq!(saturating_debit_5(6, 0), 1);
    assert_eq!(saturating_debit_5(65535, 1), 65529);
    assert_eq!(saturating_debit_5(20, 2), 13);
    assert_eq!(saturating_debit_5(40, 0), 35);
    assert_eq!(saturating_debit_5(50, 1), 44);
    assert_eq!(saturating_debit_5(100, 2), 93);
    assert_eq!(saturating_debit_5(101, 0), 96);
    assert_eq!(saturating_debit_5(255, 1), 249);
    assert_eq!(saturating_debit_5(256, 2), 249);
    assert_eq!(saturating_debit_5(0, 9), 0);
    assert_eq!(saturating_debit_5(100, 0), 95);
    assert_eq!(saturating_debit_5(100, 1), 94);
    assert_eq!(saturating_debit_5(0, 0), 0);
    assert_eq!(saturating_debit_5(5, 0), 0);
    assert_eq!(saturating_debit_5(7, 0), 2);
    assert_eq!(saturating_debit_5(8, 0), 3);
    assert_eq!(saturating_debit_5(10, 0), 5);
    assert_eq!(saturating_debit_5(11, 0), 6);
    assert_eq!(saturating_debit_5(20, 0), 15);
}

#[test]
fn submission_marker() {
    assert_eq!(task_136::TRACK_PROOF, 0x52540000 | 136);
}
