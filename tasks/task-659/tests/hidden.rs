use task_659::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_marks_removed_colon(Event::Data("")), Some(0));
    assert_eq!(closing_marks_removed_colon(Event::Data(":")), Some(0));
    assert_eq!(closing_marks_removed_colon(Event::Data("cat")), Some(3));
    assert_eq!(
        closing_marks_removed_colon(Event::Data("::one:猫:ab")),
        Some(12)
    );
    assert_eq!(closing_marks_removed_colon(Event::Data("aa:bb")), Some(5));
    assert_eq!(closing_marks_removed_colon(Event::Data("ok:🐈")), Some(7));
    assert_eq!(closing_marks_removed_colon(Event::Heartbeat), Some(0));
    assert_eq!(closing_marks_removed_colon(Event::End), None);
}

#[test]
fn submission_marker() {
    assert_eq!(task_659::TRACK_PROOF, 0x52540000 | 659);
}
