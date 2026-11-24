use task_694::*;

#[test]
fn edge_cases() {
    assert_eq!(
        shortest_field_with_first_tie_equals(Event::Data("猫=12;note")),
        Some(2)
    );
    assert_eq!(
        shortest_field_with_first_tie_equals(Event::Data("hi=kit")),
        Some(2)
    );
    assert_eq!(
        shortest_field_with_first_tie_equals(Event::Data("")),
        Some(0)
    );
    assert_eq!(
        shortest_field_with_first_tie_equals(Event::Data("=")),
        Some(0)
    );
    assert_eq!(
        shortest_field_with_first_tie_equals(Event::Data("cat")),
        Some(3)
    );
    assert_eq!(
        shortest_field_with_first_tie_equals(Event::Data("==one=猫=ab")),
        Some(0)
    );
    assert_eq!(
        shortest_field_with_first_tie_equals(Event::Data("aa=bb")),
        Some(2)
    );
    assert_eq!(
        shortest_field_with_first_tie_equals(Event::Data("ok=🐈")),
        Some(2)
    );
    assert_eq!(
        shortest_field_with_first_tie_equals(Event::Data("aa=bb;meta=123")),
        Some(2)
    );
    assert_eq!(
        shortest_field_with_first_tie_equals(Event::Data(";ignore=all")),
        Some(0)
    );
    assert_eq!(
        shortest_field_with_first_tie_equals(Event::Data("a==b;")),
        Some(0)
    );
    assert_eq!(
        shortest_field_with_first_tie_equals(Event::Heartbeat),
        Some(0)
    );
    assert_eq!(shortest_field_with_first_tie_equals(Event::End), None);
}

#[test]
fn submission_marker() {
    assert_eq!(task_694::TRACK_PROOF, 0x52540000 | 694);
}
