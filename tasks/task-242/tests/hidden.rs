use task_242::*;

#[test]
fn edge_cases() {
    assert_eq!(skip_reserved_seats_8(0, 2), 2);
    assert_eq!(skip_reserved_seats_8(1, 3), 2);
    assert_eq!(skip_reserved_seats_8(7, 4), 3);
    assert_eq!(skip_reserved_seats_8(8, 5), 3);
    assert_eq!(skip_reserved_seats_8(9, 6), 3);
    assert_eq!(skip_reserved_seats_8(255, 0), 224);
    assert_eq!(skip_reserved_seats_8(18, 3), 14);
    assert_eq!(skip_reserved_seats_8(3, 18), 14);
    assert_eq!(skip_reserved_seats_8(4, 4), 0);
    assert_eq!(skip_reserved_seats_8(0, 0), 0);
    assert_eq!(skip_reserved_seats_8(1, 0), 1);
    assert_eq!(skip_reserved_seats_8(7, 0), 7);
    assert_eq!(skip_reserved_seats_8(8, 0), 7);
    assert_eq!(skip_reserved_seats_8(9, 0), 8);
    assert_eq!(skip_reserved_seats_8(10, 0), 9);
    assert_eq!(skip_reserved_seats_8(11, 0), 10);
    assert_eq!(skip_reserved_seats_8(16, 0), 14);
    assert_eq!(skip_reserved_seats_8(17, 0), 15);
    assert_eq!(skip_reserved_seats_8(20, 0), 18);
    assert_eq!(skip_reserved_seats_8(100, 0), 88);
}

#[test]
fn submission_marker() {
    assert_eq!(task_242::TRACK_PROOF, 0x52540000 | 242);
}
