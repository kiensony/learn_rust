use task_102::*;

#[test]
fn edge_cases() {
    assert_eq!(opening_balance_10(65535, 0), 10);
    assert_eq!(opening_balance_10(1, 2), 11);
    assert_eq!(opening_balance_10(65535, 9), 19);
    assert_eq!(opening_balance_10(10, 11), 20);
    assert_eq!(opening_balance_10(65535, 11), 21);
    assert_eq!(opening_balance_10(65535, 65535), 65545);
    assert_eq!(opening_balance_10(1, 0), 10);
    assert_eq!(opening_balance_10(0, 1), 10);
    assert_eq!(opening_balance_10(0, 0), 10);
    assert_eq!(opening_balance_10(1, 1), 11);
    assert_eq!(opening_balance_10(9, 9), 19);
    assert_eq!(opening_balance_10(10, 10), 20);
    assert_eq!(opening_balance_10(11, 11), 21);
    assert_eq!(opening_balance_10(12, 12), 22);
    assert_eq!(opening_balance_10(13, 13), 23);
    assert_eq!(opening_balance_10(20, 20), 30);
    assert_eq!(opening_balance_10(21, 21), 31);
    assert_eq!(opening_balance_10(100, 100), 110);
}

#[test]
fn submission_marker() {
    assert_eq!(task_102::TRACK_PROOF, 0x52540000 | 102);
}
