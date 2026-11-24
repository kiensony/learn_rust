use task_110::*;

#[test]
fn edge_cases() {
    assert_eq!(ticket_text_10(65535, 0), "B-10/0");
    assert_eq!(ticket_text_10(1, 2), "B-10/1");
    assert_eq!(ticket_text_10(65535, 9), "B-10/9");
    assert_eq!(ticket_text_10(10, 11), "B-10/10");
    assert_eq!(ticket_text_10(65535, 11), "B-10/11");
    assert_eq!(ticket_text_10(65535, 65535), "B-10/65535");
    assert_eq!(ticket_text_10(1, 0), "B-10/0");
    assert_eq!(ticket_text_10(0, 1), "B-10/0");
    assert_eq!(ticket_text_10(0, 0), "B-10/0");
    assert_eq!(ticket_text_10(1, 1), "B-10/1");
    assert_eq!(ticket_text_10(9, 9), "B-10/9");
    assert_eq!(ticket_text_10(10, 10), "B-10/10");
    assert_eq!(ticket_text_10(11, 11), "B-10/11");
    assert_eq!(ticket_text_10(12, 12), "B-10/12");
    assert_eq!(ticket_text_10(13, 13), "B-10/13");
    assert_eq!(ticket_text_10(20, 20), "B-10/20");
    assert_eq!(ticket_text_10(21, 21), "B-10/21");
}

#[test]
fn submission_marker() {
    assert_eq!(task_110::TRACK_PROOF, 0x52540000 | 110);
}
