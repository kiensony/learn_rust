use task_581::*;

#[test]
fn edge_cases() {
    assert_eq!(marks_removed_at_both_ends_pipe("hi|kit"), "hi|kit");
    assert_eq!(marks_removed_at_both_ends_pipe("|é|a"), "é|a");
    assert_eq!(marks_removed_at_both_ends_pipe(""), "");
    assert_eq!(marks_removed_at_both_ends_pipe("|"), "");
    assert_eq!(marks_removed_at_both_ends_pipe("cat"), "cat");
    assert_eq!(marks_removed_at_both_ends_pipe("||one|猫|ab"), "one|猫|ab");
    assert_eq!(marks_removed_at_both_ends_pipe("aa|bb"), "aa|bb");
    assert_eq!(marks_removed_at_both_ends_pipe("ok|🐈"), "ok|🐈");
    assert_eq!(
        marks_removed_at_both_ends_pipe("header\nleft|right\r\n"),
        "left|right"
    );
    assert_eq!(marks_removed_at_both_ends_pipe("header\n\n"), "");
    assert_eq!(marks_removed_at_both_ends_pipe("猫|x\nend|a"), "end|a");
}

#[test]
fn submission_marker() {
    assert_eq!(task_581::TRACK_PROOF, 0x52540000 | 581);
}
