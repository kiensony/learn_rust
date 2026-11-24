use task_685::*;

#[test]
fn edge_cases() {
    assert_eq!(leading_field_equals(Event::Data("hi=kit")), Some(2));
    assert_eq!(leading_field_equals(Event::Data("=é=a")), Some(0));
    assert_eq!(leading_field_equals(Event::Data("")), Some(0));
    assert_eq!(leading_field_equals(Event::Data("=")), Some(0));
    assert_eq!(leading_field_equals(Event::Data("cat")), Some(3));
    assert_eq!(leading_field_equals(Event::Data("==one=猫=ab")), Some(0));
    assert_eq!(leading_field_equals(Event::Data("aa=bb")), Some(2));
    assert_eq!(leading_field_equals(Event::Data("ok=🐈")), Some(2));
    assert_eq!(leading_field_equals(Event::Data("aa=bb;meta=123")), Some(2));
    assert_eq!(leading_field_equals(Event::Data(";ignore=all")), Some(0));
    assert_eq!(leading_field_equals(Event::Data("a==b;")), Some(1));
    assert_eq!(leading_field_equals(Event::Heartbeat), Some(0));
    assert_eq!(leading_field_equals(Event::End), None);
}

#[test]
fn submission_marker() {
    assert_eq!(task_685::TRACK_PROOF, 0x52540000 | 685);
}
