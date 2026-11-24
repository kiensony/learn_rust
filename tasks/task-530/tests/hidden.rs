use task_530::*;

#[test]
fn edge_cases() {
    assert_eq!(leading_marks_removed_pipe("hi|kit"), 6);
    assert_eq!(leading_marks_removed_pipe("|é|a"), 3);
    assert_eq!(leading_marks_removed_pipe(""), 0);
    assert_eq!(leading_marks_removed_pipe("|"), 0);
    assert_eq!(leading_marks_removed_pipe("cat"), 3);
    assert_eq!(leading_marks_removed_pipe("||one|猫|ab"), 8);
    assert_eq!(leading_marks_removed_pipe("aa|bb"), 5);
    assert_eq!(leading_marks_removed_pipe("ok|🐈"), 4);
    assert_eq!(leading_marks_removed_pipe("header\nleft|right\r\n"), 10);
    assert_eq!(leading_marks_removed_pipe("header\n\n"), 0);
    assert_eq!(leading_marks_removed_pipe("猫|x\nend|a"), 5);
}

#[test]
fn submission_marker() {
    assert_eq!(task_530::TRACK_PROOF, 0x52540000 | 530);
}
