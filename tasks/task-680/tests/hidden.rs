use task_680::*;

#[test]
fn edge_cases() {
    assert_eq!(
        remainder_after_first_mark_pipe(Event::Data("hi|kit")),
        Some(3)
    );
    assert_eq!(
        remainder_after_first_mark_pipe(Event::Data("|é|a")),
        Some(4)
    );
    assert_eq!(remainder_after_first_mark_pipe(Event::Data("")), Some(0));
    assert_eq!(remainder_after_first_mark_pipe(Event::Data("|")), Some(0));
    assert_eq!(remainder_after_first_mark_pipe(Event::Data("cat")), Some(0));
    assert_eq!(
        remainder_after_first_mark_pipe(Event::Data("||one|猫|ab")),
        Some(11)
    );
    assert_eq!(
        remainder_after_first_mark_pipe(Event::Data("aa|bb")),
        Some(2)
    );
    assert_eq!(
        remainder_after_first_mark_pipe(Event::Data("ok|🐈")),
        Some(4)
    );
    assert_eq!(
        remainder_after_first_mark_pipe(Event::Data("header\nleft|right\r\n")),
        Some(5)
    );
    assert_eq!(
        remainder_after_first_mark_pipe(Event::Data("header\n\n")),
        Some(0)
    );
    assert_eq!(
        remainder_after_first_mark_pipe(Event::Data("猫|x\nend|a")),
        Some(1)
    );
    assert_eq!(remainder_after_first_mark_pipe(Event::Heartbeat), Some(0));
    assert_eq!(remainder_after_first_mark_pipe(Event::End), None);
}

#[test]
fn submission_marker() {
    assert_eq!(task_680::TRACK_PROOF, 0x52540000 | 680);
}
