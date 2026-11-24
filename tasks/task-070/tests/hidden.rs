use task_070::*;

#[test]
fn edge_cases() {
    assert_eq!(ticket_text_2(0), "B-2/0");
    assert_eq!(ticket_text_2(1), "B-2/1");
    assert_eq!(ticket_text_2(1), "B-2/1");
    assert_eq!(ticket_text_2(2), "B-2/2");
    assert_eq!(ticket_text_2(3), "B-2/3");
    assert_eq!(ticket_text_2(65535), "B-2/65535");
}

#[test]
fn submission_marker() {
    assert_eq!(task_070::TRACK_PROOF, 0x52540000 | 70);
}
