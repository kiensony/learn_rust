use task_100::*;

#[test]
fn edge_cases() {
    assert_eq!(ticket_text_8(0, 2), "B-8/2");
    assert_eq!(ticket_text_8(1, 3), "B-8/2");
    assert_eq!(ticket_text_8(7, 4), "B-8/3");
    assert_eq!(ticket_text_8(8, 5), "B-8/3");
    assert_eq!(ticket_text_8(9, 6), "B-8/3");
    assert_eq!(ticket_text_8(65535, 0), "B-8/65535");
    assert_eq!(ticket_text_8(18, 3), "B-8/15");
    assert_eq!(ticket_text_8(3, 18), "B-8/15");
    assert_eq!(ticket_text_8(4, 4), "B-8/0");
    assert_eq!(ticket_text_8(0, 0), "B-8/0");
    assert_eq!(ticket_text_8(1, 0), "B-8/1");
    assert_eq!(ticket_text_8(7, 0), "B-8/7");
    assert_eq!(ticket_text_8(8, 0), "B-8/8");
    assert_eq!(ticket_text_8(9, 0), "B-8/9");
    assert_eq!(ticket_text_8(10, 0), "B-8/10");
    assert_eq!(ticket_text_8(11, 0), "B-8/11");
    assert_eq!(ticket_text_8(16, 0), "B-8/16");
    assert_eq!(ticket_text_8(17, 0), "B-8/17");
    assert_eq!(ticket_text_8(20, 0), "B-8/20");
    assert_eq!(ticket_text_8(100, 0), "B-8/100");
}

#[test]
fn submission_marker() {
    assert_eq!(task_100::TRACK_PROOF, 0x52540000 | 100);
}
