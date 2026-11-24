use task_675::*;

#[test]
fn edge_cases() {
    assert_eq!(leading_field_pipe(Event::Data("header\n猫|123")), Some(3));
    assert_eq!(leading_field_pipe(Event::Data("|é|a")), Some(0));
    assert_eq!(leading_field_pipe(Event::Data("")), Some(0));
    assert_eq!(leading_field_pipe(Event::Data("|")), Some(0));
    assert_eq!(leading_field_pipe(Event::Data("cat")), Some(3));
    assert_eq!(leading_field_pipe(Event::Data("||one|猫|ab")), Some(0));
    assert_eq!(leading_field_pipe(Event::Data("aa|bb")), Some(2));
    assert_eq!(leading_field_pipe(Event::Data("ok|🐈")), Some(2));
    assert_eq!(
        leading_field_pipe(Event::Data("header\nleft|right\r\n")),
        Some(4)
    );
    assert_eq!(leading_field_pipe(Event::Data("header\n\n")), Some(0));
    assert_eq!(leading_field_pipe(Event::Data("猫|x\nend|a")), Some(3));
    assert_eq!(leading_field_pipe(Event::Heartbeat), Some(0));
    assert_eq!(leading_field_pipe(Event::End), None);
}

#[test]
fn submission_marker() {
    assert_eq!(task_675::TRACK_PROOF, 0x52540000 | 675);
}
