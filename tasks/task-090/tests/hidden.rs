use task_090::*;

#[test]
fn edge_cases() {
    assert_eq!(ticket_text_5(0, 2), "B-5/0");
    assert_eq!(ticket_text_5(1, 0), "B-5/1");
    assert_eq!(ticket_text_5(4, 1), "B-5/3");
    assert_eq!(ticket_text_5(5, 2), "B-5/3");
    assert_eq!(ticket_text_5(6, 0), "B-5/6");
    assert_eq!(ticket_text_5(65535, 1), "B-5/65534");
    assert_eq!(ticket_text_5(0, 9), "B-5/0");
    assert_eq!(ticket_text_5(100, 0), "B-5/100");
    assert_eq!(ticket_text_5(100, 1), "B-5/99");
    assert_eq!(ticket_text_5(0, 0), "B-5/0");
    assert_eq!(ticket_text_5(5, 0), "B-5/5");
    assert_eq!(ticket_text_5(7, 0), "B-5/7");
    assert_eq!(ticket_text_5(8, 0), "B-5/8");
    assert_eq!(ticket_text_5(10, 0), "B-5/10");
    assert_eq!(ticket_text_5(11, 0), "B-5/11");
    assert_eq!(ticket_text_5(20, 0), "B-5/20");
}

#[test]
fn submission_marker() {
    assert_eq!(task_090::TRACK_PROOF, 0x52540000 | 90);
}
