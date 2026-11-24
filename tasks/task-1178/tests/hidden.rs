use task_1178::*;

#[test]
fn edge_cases() {
    assert_eq!(first_nonempty_field_pipe("hi|kit", "backup"), "hi");
    assert_eq!(first_nonempty_field_pipe("|é|a", "backup"), "é");
    assert_eq!(first_nonempty_field_pipe("", "backup"), "backup");
    assert_eq!(first_nonempty_field_pipe("|", "backup"), "backup");
    assert_eq!(first_nonempty_field_pipe("cat", "backup"), "cat");
    assert_eq!(first_nonempty_field_pipe("||one|猫|ab", "backup"), "one");
    assert_eq!(first_nonempty_field_pipe("aa|bb", "backup"), "aa");
    assert_eq!(first_nonempty_field_pipe("ok|🐈", "backup"), "ok");
    assert_eq!(
        first_nonempty_field_pipe("header\nleft|right\r\n", "backup"),
        "left"
    );
    assert_eq!(first_nonempty_field_pipe("header\n\n", "backup"), "backup");
    assert_eq!(first_nonempty_field_pipe("猫|x\nend|a", "backup"), "end");
}

#[test]
fn submission_marker() {
    assert_eq!(task_1178::TRACK_PROOF, 0x52540000 | 1178);
}
