use task_1173::*;

#[test]
fn edge_cases() {
    assert_eq!(leading_marks_removed_pipe("hi|kit", "backup"), "hi|kit");
    assert_eq!(leading_marks_removed_pipe("|é|a", "backup"), "é|a");
    assert_eq!(leading_marks_removed_pipe("", "backup"), "backup");
    assert_eq!(leading_marks_removed_pipe("|", "backup"), "backup");
    assert_eq!(leading_marks_removed_pipe("cat", "backup"), "cat");
    assert_eq!(
        leading_marks_removed_pipe("||one|猫|ab", "backup"),
        "one|猫|ab"
    );
    assert_eq!(leading_marks_removed_pipe("aa|bb", "backup"), "aa|bb");
    assert_eq!(leading_marks_removed_pipe("ok|🐈", "backup"), "ok|🐈");
    assert_eq!(
        leading_marks_removed_pipe("header\nleft|right\r\n", "backup"),
        "left|right"
    );
    assert_eq!(leading_marks_removed_pipe("header\n\n", "backup"), "backup");
    assert_eq!(leading_marks_removed_pipe("猫|x\nend|a", "backup"), "end|a");
}

#[test]
fn submission_marker() {
    assert_eq!(task_1173::TRACK_PROOF, 0x52540000 | 1173);
}
