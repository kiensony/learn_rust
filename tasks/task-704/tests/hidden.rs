use task_704::*;

#[test]
fn edge_cases() {
    assert_eq!(
        shortest_field_with_first_tie_hash(Event::Data("unclosed[cat")),
        Some(0)
    );
    assert_eq!(
        shortest_field_with_first_tie_hash(Event::Data("[]")),
        Some(0)
    );
    assert_eq!(
        shortest_field_with_first_tie_hash(Event::Data("[42]")),
        Some(2)
    );
    assert_eq!(
        shortest_field_with_first_tie_hash(Event::Data("[321]")),
        Some(3)
    );
    assert_eq!(
        shortest_field_with_first_tie_hash(Event::Data("hi#kit")),
        Some(0)
    );
    assert_eq!(
        shortest_field_with_first_tie_hash(Event::Data("#é#a")),
        Some(0)
    );
    assert_eq!(shortest_field_with_first_tie_hash(Event::Data("")), Some(0));
    assert_eq!(
        shortest_field_with_first_tie_hash(Event::Data("#")),
        Some(0)
    );
    assert_eq!(
        shortest_field_with_first_tie_hash(Event::Data("cat")),
        Some(0)
    );
    assert_eq!(
        shortest_field_with_first_tie_hash(Event::Data("##one#猫#ab")),
        Some(0)
    );
    assert_eq!(
        shortest_field_with_first_tie_hash(Event::Data("aa#bb")),
        Some(0)
    );
    assert_eq!(
        shortest_field_with_first_tie_hash(Event::Data("ok#🐈")),
        Some(0)
    );
    assert_eq!(
        shortest_field_with_first_tie_hash(Event::Heartbeat),
        Some(0)
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_704::TRACK_PROOF, 0x52540000 | 704);
}
