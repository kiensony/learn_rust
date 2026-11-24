use task_1179::*;

#[test]
fn edge_cases() {
    assert_eq!(
        longest_field_with_last_tie_pipe("header\n猫|123", "backup"),
        "123"
    );
    assert_eq!(longest_field_with_last_tie_pipe("|é|a", "backup"), "é");
    assert_eq!(longest_field_with_last_tie_pipe("", "backup"), "backup");
    assert_eq!(longest_field_with_last_tie_pipe("|", "backup"), "backup");
    assert_eq!(longest_field_with_last_tie_pipe("cat", "backup"), "cat");
    assert_eq!(
        longest_field_with_last_tie_pipe("||one|猫|ab", "backup"),
        "猫"
    );
    assert_eq!(longest_field_with_last_tie_pipe("aa|bb", "backup"), "bb");
    assert_eq!(longest_field_with_last_tie_pipe("ok|🐈", "backup"), "🐈");
    assert_eq!(
        longest_field_with_last_tie_pipe("header\nleft|right\r\n", "backup"),
        "right"
    );
    assert_eq!(
        longest_field_with_last_tie_pipe("header\n\n", "backup"),
        "backup"
    );
    assert_eq!(
        longest_field_with_last_tie_pipe("猫|x\nend|a", "backup"),
        "end"
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1179::TRACK_PROOF, 0x52540000 | 1179);
}
