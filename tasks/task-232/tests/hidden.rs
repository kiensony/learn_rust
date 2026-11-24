use task_232::*;

#[test]
fn edge_cases() {
    assert_eq!(skip_reserved_seats_5(0, 2), 0);
    assert_eq!(skip_reserved_seats_5(1, 0), 1);
    assert_eq!(skip_reserved_seats_5(4, 1), 3);
    assert_eq!(skip_reserved_seats_5(5, 2), 3);
    assert_eq!(skip_reserved_seats_5(6, 0), 5);
    assert_eq!(skip_reserved_seats_5(255, 1), 204);
    assert_eq!(skip_reserved_seats_5(0, 9), 0);
    assert_eq!(skip_reserved_seats_5(100, 0), 80);
    assert_eq!(skip_reserved_seats_5(100, 1), 80);
    assert_eq!(skip_reserved_seats_5(0, 0), 0);
    assert_eq!(skip_reserved_seats_5(5, 0), 4);
    assert_eq!(skip_reserved_seats_5(7, 0), 6);
    assert_eq!(skip_reserved_seats_5(8, 0), 7);
    assert_eq!(skip_reserved_seats_5(10, 0), 8);
    assert_eq!(skip_reserved_seats_5(11, 0), 9);
    assert_eq!(skip_reserved_seats_5(20, 0), 16);
}

#[test]
fn submission_marker() {
    assert_eq!(task_232::TRACK_PROOF, 0x52540000 | 232);
}
