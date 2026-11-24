use task_660::*;

#[test]
fn edge_cases() {
    assert_eq!(marks_removed_at_both_ends_colon(Event::Data("")), Some(0));
    assert_eq!(marks_removed_at_both_ends_colon(Event::Data(":")), Some(0));
    assert_eq!(
        marks_removed_at_both_ends_colon(Event::Data("cat")),
        Some(3)
    );
    assert_eq!(
        marks_removed_at_both_ends_colon(Event::Data("::one:猫:ab")),
        Some(10)
    );
    assert_eq!(
        marks_removed_at_both_ends_colon(Event::Data("aa:bb")),
        Some(5)
    );
    assert_eq!(
        marks_removed_at_both_ends_colon(Event::Data("ok:🐈")),
        Some(7)
    );
    assert_eq!(marks_removed_at_both_ends_colon(Event::Heartbeat), Some(0));
    assert_eq!(marks_removed_at_both_ends_colon(Event::End), None);
}

#[test]
fn submission_marker() {
    assert_eq!(task_660::TRACK_PROOF, 0x52540000 | 660);
}
