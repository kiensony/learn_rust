use task_585::*;

#[test]
fn edge_cases() {
    assert_eq!(longest_field_with_last_tie_pipe("header\n猫|123"), "123");
    assert_eq!(longest_field_with_last_tie_pipe("|é|a"), "é");
    assert_eq!(longest_field_with_last_tie_pipe(""), "");
    assert_eq!(longest_field_with_last_tie_pipe("|"), "");
    assert_eq!(longest_field_with_last_tie_pipe("cat"), "cat");
    assert_eq!(longest_field_with_last_tie_pipe("||one|猫|ab"), "猫");
    assert_eq!(longest_field_with_last_tie_pipe("aa|bb"), "bb");
    assert_eq!(longest_field_with_last_tie_pipe("ok|🐈"), "🐈");
    assert_eq!(
        longest_field_with_last_tie_pipe("header\nleft|right\r\n"),
        "right"
    );
    assert_eq!(longest_field_with_last_tie_pipe("header\n\n"), "");
    assert_eq!(longest_field_with_last_tie_pipe("猫|x\nend|a"), "end");
}

#[test]
fn submission_marker() {
    assert_eq!(task_585::TRACK_PROOF, 0x52540000 | 585);
}
