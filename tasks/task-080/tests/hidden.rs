use task_080::*;

#[test]
fn edge_cases() {
    assert_eq!(ticket_text_3(0, 6), "B-3/6");
    assert_eq!(ticket_text_3(1, 9), "B-3/10");
    assert_eq!(ticket_text_3(2, 12), "B-3/14");
    assert_eq!(ticket_text_3(3, 15), "B-3/18");
    assert_eq!(ticket_text_3(4, 1), "B-3/5");
    assert_eq!(ticket_text_3(65535, 4), "B-3/65535");
    assert_eq!(ticket_text_3(65535, 1), "B-3/65535");
    assert_eq!(ticket_text_3(0, 0), "B-3/0");
    assert_eq!(ticket_text_3(1, 0), "B-3/1");
    assert_eq!(ticket_text_3(2, 0), "B-3/2");
    assert_eq!(ticket_text_3(3, 0), "B-3/3");
    assert_eq!(ticket_text_3(5, 0), "B-3/5");
    assert_eq!(ticket_text_3(6, 0), "B-3/6");
    assert_eq!(ticket_text_3(7, 0), "B-3/7");
    assert_eq!(ticket_text_3(20, 0), "B-3/20");
    assert_eq!(ticket_text_3(100, 0), "B-3/100");
}

#[test]
fn submission_marker() {
    assert_eq!(task_080::TRACK_PROOF, 0x52540000 | 80);
}
