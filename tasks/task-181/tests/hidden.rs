use task_181::*;

#[test]
fn edge_cases() {
    assert_eq!(last_ticket_5(0, 2), true);
    assert_eq!(last_ticket_5(1, 0), true);
    assert_eq!(last_ticket_5(4, 1), true);
    assert_eq!(last_ticket_5(5, 2), true);
    assert_eq!(last_ticket_5(6, 0), false);
    assert_eq!(last_ticket_5(65535, 1), false);
    assert_eq!(last_ticket_5(7, 2), false);
    assert_eq!(last_ticket_5(8, 0), false);
    assert_eq!(last_ticket_5(10, 1), false);
    assert_eq!(last_ticket_5(11, 2), false);
    assert_eq!(last_ticket_5(0, 9), true);
    assert_eq!(last_ticket_5(100, 0), false);
    assert_eq!(last_ticket_5(100, 1), false);
    assert_eq!(last_ticket_5(0, 0), true);
    assert_eq!(last_ticket_5(5, 0), false);
    assert_eq!(last_ticket_5(7, 0), false);
    assert_eq!(last_ticket_5(10, 0), false);
    assert_eq!(last_ticket_5(11, 0), false);
    assert_eq!(last_ticket_5(20, 0), false);
}

#[test]
fn submission_marker() {
    assert_eq!(task_181::TRACK_PROOF, 0x52540000 | 181);
}
