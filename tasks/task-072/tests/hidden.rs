use task_072::*;

#[test]
fn edge_cases() {
    assert_eq!(opening_balance_3(0, 6), 9);
    assert_eq!(opening_balance_3(1, 9), 13);
    assert_eq!(opening_balance_3(2, 12), 17);
    assert_eq!(opening_balance_3(3, 15), 21);
    assert_eq!(opening_balance_3(4, 1), 8);
    assert_eq!(opening_balance_3(65535, 4), 65538);
    assert_eq!(opening_balance_3(65535, 1), 65538);
    assert_eq!(opening_balance_3(0, 0), 3);
    assert_eq!(opening_balance_3(1, 0), 4);
    assert_eq!(opening_balance_3(2, 0), 5);
    assert_eq!(opening_balance_3(3, 0), 6);
    assert_eq!(opening_balance_3(5, 0), 8);
    assert_eq!(opening_balance_3(6, 0), 9);
    assert_eq!(opening_balance_3(7, 0), 10);
    assert_eq!(opening_balance_3(20, 0), 23);
    assert_eq!(opening_balance_3(100, 0), 103);
}

#[test]
fn submission_marker() {
    assert_eq!(task_072::TRACK_PROOF, 0x52540000 | 72);
}
