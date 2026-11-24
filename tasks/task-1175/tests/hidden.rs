use task_1175::*;

#[test]
fn edge_cases() {
    assert_eq!(
        marks_removed_at_both_ends_pipe("hi|kit", "backup"),
        "hi|kit"
    );
    assert_eq!(marks_removed_at_both_ends_pipe("|é|a", "backup"), "é|a");
    assert_eq!(marks_removed_at_both_ends_pipe("", "backup"), "backup");
    assert_eq!(marks_removed_at_both_ends_pipe("|", "backup"), "backup");
    assert_eq!(marks_removed_at_both_ends_pipe("cat", "backup"), "cat");
    assert_eq!(
        marks_removed_at_both_ends_pipe("||one|猫|ab", "backup"),
        "one|猫|ab"
    );
    assert_eq!(marks_removed_at_both_ends_pipe("aa|bb", "backup"), "aa|bb");
    assert_eq!(marks_removed_at_both_ends_pipe("ok|🐈", "backup"), "ok|🐈");
    assert_eq!(
        marks_removed_at_both_ends_pipe("header\nleft|right\r\n", "backup"),
        "left|right"
    );
    assert_eq!(
        marks_removed_at_both_ends_pipe("header\n\n", "backup"),
        "backup"
    );
    assert_eq!(
        marks_removed_at_both_ends_pipe("猫|x\nend|a", "backup"),
        "end|a"
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1175::TRACK_PROOF, 0x52540000 | 1175);
}
