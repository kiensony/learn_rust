use task_285::*;

#[test]
fn edge_cases() {
    assert_eq!(next_room(0), 1);
    assert_eq!(next_room(65535), 65536);
    assert_eq!(next_room(2), 3);
}

#[test]
fn submission_marker() {
    assert_eq!(task_285::TRACK_PROOF, 0x52540000 | 285);
}
