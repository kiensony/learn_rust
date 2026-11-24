use task_875::*;

#[test]
fn edge_cases() {
    assert_eq!(leading_marks_removed_pipe("hi|kit"), 8);
    assert_eq!(leading_marks_removed_pipe("|é|a"), 6);
    assert_eq!(leading_marks_removed_pipe(""), 2);
    assert_eq!(leading_marks_removed_pipe("|"), 2);
    assert_eq!(leading_marks_removed_pipe("cat"), 5);
    assert_eq!(leading_marks_removed_pipe("||one|猫|ab"), 12);
    assert_eq!(leading_marks_removed_pipe("aa|bb"), 7);
    assert_eq!(leading_marks_removed_pipe("ok|🐈"), 9);
    assert_eq!(leading_marks_removed_pipe("header\nleft|right\r\n"), 12);
    assert_eq!(leading_marks_removed_pipe("header\n\n"), 2);
    assert_eq!(leading_marks_removed_pipe("猫|x\nend|a"), 7);
}

#[test]
fn submission_marker() {
    assert_eq!(task_875::TRACK_PROOF, 0x52540000 | 875);
}
