use task_1176::*;

#[test]
fn edge_cases() {
    assert_eq!(remainder_after_first_mark_pipe("hi|kit", "backup"), "kit");
    assert_eq!(remainder_after_first_mark_pipe("|é|a", "backup"), "é|a");
    assert_eq!(remainder_after_first_mark_pipe("", "backup"), "backup");
    assert_eq!(remainder_after_first_mark_pipe("|", "backup"), "backup");
    assert_eq!(remainder_after_first_mark_pipe("cat", "backup"), "backup");
    assert_eq!(
        remainder_after_first_mark_pipe("||one|猫|ab", "backup"),
        "|one|猫|ab"
    );
    assert_eq!(remainder_after_first_mark_pipe("aa|bb", "backup"), "bb");
    assert_eq!(remainder_after_first_mark_pipe("ok|🐈", "backup"), "🐈");
    assert_eq!(
        remainder_after_first_mark_pipe("header\nleft|right\r\n", "backup"),
        "right"
    );
    assert_eq!(
        remainder_after_first_mark_pipe("header\n\n", "backup"),
        "backup"
    );
    assert_eq!(
        remainder_after_first_mark_pipe("猫|x\nend|a", "backup"),
        "a"
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1176::TRACK_PROOF, 0x52540000 | 1176);
}
