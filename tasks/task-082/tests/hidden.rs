use task_082::*;

#[test]
fn edge_cases() {
    assert_eq!(opening_balance_5(0, 2), 5);
    assert_eq!(opening_balance_5(1, 0), 6);
    assert_eq!(opening_balance_5(4, 1), 8);
    assert_eq!(opening_balance_5(5, 2), 8);
    assert_eq!(opening_balance_5(6, 0), 11);
    assert_eq!(opening_balance_5(65535, 1), 65539);
    assert_eq!(opening_balance_5(0, 9), 5);
    assert_eq!(opening_balance_5(100, 0), 105);
    assert_eq!(opening_balance_5(100, 1), 104);
    assert_eq!(opening_balance_5(0, 0), 5);
    assert_eq!(opening_balance_5(5, 0), 10);
    assert_eq!(opening_balance_5(7, 0), 12);
    assert_eq!(opening_balance_5(8, 0), 13);
    assert_eq!(opening_balance_5(10, 0), 15);
    assert_eq!(opening_balance_5(11, 0), 16);
    assert_eq!(opening_balance_5(20, 0), 25);
}

#[test]
fn submission_marker() {
    assert_eq!(task_082::TRACK_PROOF, 0x52540000 | 82);
}
