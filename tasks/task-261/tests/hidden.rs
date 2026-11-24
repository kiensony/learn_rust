use task_261::*;

#[test]
fn edge_cases() {
    assert_eq!(double_tickets(0), 0);
    assert_eq!(double_tickets(1), 2);
    assert_eq!(double_tickets(65535), 131070);
}

#[test]
fn submission_marker() {
    assert_eq!(task_261::TRACK_PROOF, 0x52540000 | 261);
}
