use task_222::*;

#[test]
fn edge_cases() {
    assert_eq!(skip_reserved_seats_3(0, 6), 4);
    assert_eq!(skip_reserved_seats_3(1, 9), 7);
    assert_eq!(skip_reserved_seats_3(2, 12), 10);
    assert_eq!(skip_reserved_seats_3(3, 15), 12);
    assert_eq!(skip_reserved_seats_3(4, 1), 4);
    assert_eq!(skip_reserved_seats_3(255, 4), 170);
    assert_eq!(skip_reserved_seats_3(255, 1), 170);
    assert_eq!(skip_reserved_seats_3(0, 0), 0);
    assert_eq!(skip_reserved_seats_3(1, 0), 1);
    assert_eq!(skip_reserved_seats_3(2, 0), 2);
    assert_eq!(skip_reserved_seats_3(3, 0), 2);
    assert_eq!(skip_reserved_seats_3(5, 0), 4);
    assert_eq!(skip_reserved_seats_3(6, 0), 4);
    assert_eq!(skip_reserved_seats_3(7, 0), 5);
    assert_eq!(skip_reserved_seats_3(20, 0), 14);
    assert_eq!(skip_reserved_seats_3(100, 0), 67);
}

#[test]
fn submission_marker() {
    assert_eq!(task_222::TRACK_PROOF, 0x52540000 | 222);
}
