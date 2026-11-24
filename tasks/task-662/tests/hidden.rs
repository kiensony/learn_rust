use task_662::*;

#[test]
fn edge_cases() {
    assert_eq!(first_nonempty_field_colon(Event::Data(":é:a")), Some(2));
    assert_eq!(first_nonempty_field_colon(Event::Data(":")), Some(0));
    assert_eq!(first_nonempty_field_colon(Event::Data("cat")), Some(3));
    assert_eq!(
        first_nonempty_field_colon(Event::Data("::one:猫:ab")),
        Some(3)
    );
    assert_eq!(first_nonempty_field_colon(Event::Data("aa:bb")), Some(2));
    assert_eq!(first_nonempty_field_colon(Event::Data("ok:🐈")), Some(2));
    assert_eq!(first_nonempty_field_colon(Event::Heartbeat), Some(0));
    assert_eq!(first_nonempty_field_colon(Event::End), None);
}

#[test]
fn submission_marker() {
    assert_eq!(task_662::TRACK_PROOF, 0x52540000 | 662);
}
