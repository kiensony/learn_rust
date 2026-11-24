use task_579::*;

#[test]
fn edge_cases() {
    assert_eq!(leading_marks_removed_pipe("hi|kit"), "hi|kit");
    assert_eq!(leading_marks_removed_pipe("|é|a"), "é|a");
    assert_eq!(leading_marks_removed_pipe(""), "");
    assert_eq!(leading_marks_removed_pipe("|"), "");
    assert_eq!(leading_marks_removed_pipe("cat"), "cat");
    assert_eq!(leading_marks_removed_pipe("||one|猫|ab"), "one|猫|ab");
    assert_eq!(leading_marks_removed_pipe("aa|bb"), "aa|bb");
    assert_eq!(leading_marks_removed_pipe("ok|🐈"), "ok|🐈");
    assert_eq!(
        leading_marks_removed_pipe("header\nleft|right\r\n"),
        "left|right"
    );
    assert_eq!(leading_marks_removed_pipe("header\n\n"), "");
    assert_eq!(leading_marks_removed_pipe("猫|x\nend|a"), "end|a");
}

#[test]
fn submission_marker() {
    assert_eq!(task_579::TRACK_PROOF, 0x52540000 | 579);
}
