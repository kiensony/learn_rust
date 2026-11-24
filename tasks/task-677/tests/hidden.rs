use task_677::*;

#[test]
fn edge_cases() {
    assert_eq!(leading_marks_removed_pipe(Event::Data("hi|kit")), Some(6));
    assert_eq!(leading_marks_removed_pipe(Event::Data("|é|a")), Some(4));
    assert_eq!(leading_marks_removed_pipe(Event::Data("")), Some(0));
    assert_eq!(leading_marks_removed_pipe(Event::Data("|")), Some(0));
    assert_eq!(leading_marks_removed_pipe(Event::Data("cat")), Some(3));
    assert_eq!(
        leading_marks_removed_pipe(Event::Data("||one|猫|ab")),
        Some(10)
    );
    assert_eq!(leading_marks_removed_pipe(Event::Data("aa|bb")), Some(5));
    assert_eq!(leading_marks_removed_pipe(Event::Data("ok|🐈")), Some(7));
    assert_eq!(
        leading_marks_removed_pipe(Event::Data("header\nleft|right\r\n")),
        Some(10)
    );
    assert_eq!(
        leading_marks_removed_pipe(Event::Data("header\n\n")),
        Some(0)
    );
    assert_eq!(
        leading_marks_removed_pipe(Event::Data("猫|x\nend|a")),
        Some(5)
    );
    assert_eq!(leading_marks_removed_pipe(Event::Heartbeat), Some(0));
    assert_eq!(leading_marks_removed_pipe(Event::End), None);
}

#[test]
fn submission_marker() {
    assert_eq!(task_677::TRACK_PROOF, 0x52540000 | 677);
}
