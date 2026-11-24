use task_695::*;

#[test]
fn edge_cases() {
    assert_eq!(leading_field_hash(Event::Data("[猫###a]")), Some(3));
    assert_eq!(leading_field_hash(Event::Data("[]")), Some(0));
    assert_eq!(leading_field_hash(Event::Data("[42]")), Some(2));
    assert_eq!(leading_field_hash(Event::Data("[321]")), Some(3));
    assert_eq!(leading_field_hash(Event::Data("hi#kit")), Some(0));
    assert_eq!(leading_field_hash(Event::Data("#é#a")), Some(0));
    assert_eq!(leading_field_hash(Event::Data("")), Some(0));
    assert_eq!(leading_field_hash(Event::Data("#")), Some(0));
    assert_eq!(leading_field_hash(Event::Data("cat")), Some(0));
    assert_eq!(leading_field_hash(Event::Data("##one#猫#ab")), Some(0));
    assert_eq!(leading_field_hash(Event::Data("aa#bb")), Some(0));
    assert_eq!(leading_field_hash(Event::Data("ok#🐈")), Some(0));
    assert_eq!(leading_field_hash(Event::Heartbeat), Some(0));
    assert_eq!(leading_field_hash(Event::End), None);
}

#[test]
fn submission_marker() {
    assert_eq!(task_695::TRACK_PROOF, 0x52540000 | 695);
}
