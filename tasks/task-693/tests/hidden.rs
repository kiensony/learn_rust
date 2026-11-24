use task_693::*;

#[test]
fn edge_cases() {
    assert_eq!(
        longest_field_with_last_tie_equals(Event::Data("hi=kit")),
        Some(3)
    );
    assert_eq!(
        longest_field_with_last_tie_equals(Event::Data("=é=a")),
        Some(2)
    );
    assert_eq!(longest_field_with_last_tie_equals(Event::Data("")), Some(0));
    assert_eq!(
        longest_field_with_last_tie_equals(Event::Data("=")),
        Some(0)
    );
    assert_eq!(
        longest_field_with_last_tie_equals(Event::Data("cat")),
        Some(3)
    );
    assert_eq!(
        longest_field_with_last_tie_equals(Event::Data("==one=猫=ab")),
        Some(3)
    );
    assert_eq!(
        longest_field_with_last_tie_equals(Event::Data("aa=bb")),
        Some(2)
    );
    assert_eq!(
        longest_field_with_last_tie_equals(Event::Data("ok=🐈")),
        Some(4)
    );
    assert_eq!(
        longest_field_with_last_tie_equals(Event::Data("aa=bb;meta=123")),
        Some(2)
    );
    assert_eq!(
        longest_field_with_last_tie_equals(Event::Data(";ignore=all")),
        Some(0)
    );
    assert_eq!(
        longest_field_with_last_tie_equals(Event::Data("a==b;")),
        Some(1)
    );
    assert_eq!(
        longest_field_with_last_tie_equals(Event::Heartbeat),
        Some(0)
    );
    assert_eq!(longest_field_with_last_tie_equals(Event::End), None);
}

#[test]
fn submission_marker() {
    assert_eq!(task_693::TRACK_PROOF, 0x52540000 | 693);
}
