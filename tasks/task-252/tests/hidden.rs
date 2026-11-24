use task_252::*;

#[test]
fn edge_cases() {
    assert_eq!(skip_reserved_seats_10(255, 0), 0);
    assert_eq!(skip_reserved_seats_10(1, 2), 1);
    assert_eq!(skip_reserved_seats_10(255, 9), 9);
    assert_eq!(skip_reserved_seats_10(10, 11), 9);
    assert_eq!(skip_reserved_seats_10(255, 11), 10);
    assert_eq!(skip_reserved_seats_10(255, 255), 230);
    assert_eq!(skip_reserved_seats_10(1, 0), 0);
    assert_eq!(skip_reserved_seats_10(0, 1), 0);
    assert_eq!(skip_reserved_seats_10(0, 0), 0);
    assert_eq!(skip_reserved_seats_10(1, 1), 1);
    assert_eq!(skip_reserved_seats_10(9, 9), 9);
    assert_eq!(skip_reserved_seats_10(10, 10), 9);
    assert_eq!(skip_reserved_seats_10(11, 11), 10);
    assert_eq!(skip_reserved_seats_10(12, 12), 11);
    assert_eq!(skip_reserved_seats_10(13, 13), 12);
    assert_eq!(skip_reserved_seats_10(20, 20), 18);
    assert_eq!(skip_reserved_seats_10(21, 21), 19);
    assert_eq!(skip_reserved_seats_10(100, 100), 90);
}

#[test]
fn submission_marker() {
    assert_eq!(task_252::TRACK_PROOF, 0x52540000 | 252);
}
