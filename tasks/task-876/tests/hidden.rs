use task_876::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_marks_removed_pipe("hi|kit"), 8);
    assert_eq!(closing_marks_removed_pipe("|é|a"), 7);
    assert_eq!(closing_marks_removed_pipe(""), 2);
    assert_eq!(closing_marks_removed_pipe("|"), 2);
    assert_eq!(closing_marks_removed_pipe("cat"), 5);
    assert_eq!(closing_marks_removed_pipe("||one|猫|ab"), 14);
    assert_eq!(closing_marks_removed_pipe("aa|bb"), 7);
    assert_eq!(closing_marks_removed_pipe("ok|🐈"), 9);
    assert_eq!(closing_marks_removed_pipe("header\nleft|right\r\n"), 12);
    assert_eq!(closing_marks_removed_pipe("header\n\n"), 2);
    assert_eq!(closing_marks_removed_pipe("猫|x\nend|a"), 7);
}

#[test]
fn submission_marker() {
    assert_eq!(task_876::TRACK_PROOF, 0x52540000 | 876);
}
