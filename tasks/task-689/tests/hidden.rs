use task_689::*;

#[test]
fn edge_cases() {
    assert_eq!(
        marks_removed_at_both_ends_equals(Event::Data("hi=kit")),
        Some(6)
    );
    assert_eq!(
        marks_removed_at_both_ends_equals(Event::Data("=é=a")),
        Some(4)
    );
    assert_eq!(marks_removed_at_both_ends_equals(Event::Data("")), Some(0));
    assert_eq!(marks_removed_at_both_ends_equals(Event::Data("=")), Some(0));
    assert_eq!(
        marks_removed_at_both_ends_equals(Event::Data("cat")),
        Some(3)
    );
    assert_eq!(
        marks_removed_at_both_ends_equals(Event::Data("==one=猫=ab")),
        Some(10)
    );
    assert_eq!(
        marks_removed_at_both_ends_equals(Event::Data("aa=bb")),
        Some(5)
    );
    assert_eq!(
        marks_removed_at_both_ends_equals(Event::Data("ok=🐈")),
        Some(7)
    );
    assert_eq!(
        marks_removed_at_both_ends_equals(Event::Data("aa=bb;meta=123")),
        Some(5)
    );
    assert_eq!(
        marks_removed_at_both_ends_equals(Event::Data(";ignore=all")),
        Some(0)
    );
    assert_eq!(
        marks_removed_at_both_ends_equals(Event::Data("a==b;")),
        Some(4)
    );
    assert_eq!(marks_removed_at_both_ends_equals(Event::Heartbeat), Some(0));
    assert_eq!(marks_removed_at_both_ends_equals(Event::End), None);
}

#[test]
fn submission_marker() {
    assert_eq!(task_689::TRACK_PROOF, 0x52540000 | 689);
}
