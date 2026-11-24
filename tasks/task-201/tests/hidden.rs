use task_201::*;

#[test]
fn edge_cases() {
    assert_eq!(last_ticket_10(65535, 0), true);
    assert_eq!(last_ticket_10(1, 2), true);
    assert_eq!(last_ticket_10(65535, 9), true);
    assert_eq!(last_ticket_10(10, 11), false);
    assert_eq!(last_ticket_10(65535, 11), false);
    assert_eq!(last_ticket_10(65535, 65535), false);
    assert_eq!(last_ticket_10(65535, 12), false);
    assert_eq!(last_ticket_10(65535, 20), false);
    assert_eq!(last_ticket_10(21, 22), false);
    assert_eq!(last_ticket_10(1, 0), true);
    assert_eq!(last_ticket_10(0, 1), true);
    assert_eq!(last_ticket_10(0, 0), true);
    assert_eq!(last_ticket_10(1, 1), true);
    assert_eq!(last_ticket_10(9, 9), true);
    assert_eq!(last_ticket_10(10, 10), false);
    assert_eq!(last_ticket_10(11, 11), false);
    assert_eq!(last_ticket_10(12, 12), false);
    assert_eq!(last_ticket_10(13, 13), false);
    assert_eq!(last_ticket_10(20, 20), false);
    assert_eq!(last_ticket_10(21, 21), false);
    assert_eq!(last_ticket_10(100, 100), false);
}

#[test]
fn submission_marker() {
    assert_eq!(task_201::TRACK_PROOF, 0x52540000 | 201);
}
