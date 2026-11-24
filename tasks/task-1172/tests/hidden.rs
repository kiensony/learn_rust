use task_1172::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_field_pipe("header\n猫|123", "backup"), "123");
    assert_eq!(closing_field_pipe("|é|a", "backup"), "a");
    assert_eq!(closing_field_pipe("", "backup"), "backup");
    assert_eq!(closing_field_pipe("|", "backup"), "backup");
    assert_eq!(closing_field_pipe("cat", "backup"), "cat");
    assert_eq!(closing_field_pipe("||one|猫|ab", "backup"), "ab");
    assert_eq!(closing_field_pipe("aa|bb", "backup"), "bb");
    assert_eq!(closing_field_pipe("ok|🐈", "backup"), "🐈");
    assert_eq!(
        closing_field_pipe("header\nleft|right\r\n", "backup"),
        "right"
    );
    assert_eq!(closing_field_pipe("header\n\n", "backup"), "backup");
    assert_eq!(closing_field_pipe("猫|x\nend|a", "backup"), "a");
}

#[test]
fn submission_marker() {
    assert_eq!(task_1172::TRACK_PROOF, 0x52540000 | 1172);
}
