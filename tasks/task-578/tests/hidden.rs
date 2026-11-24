use task_578::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_field_pipe("header\n猫|123"), "123");
    assert_eq!(closing_field_pipe("|é|a"), "a");
    assert_eq!(closing_field_pipe(""), "");
    assert_eq!(closing_field_pipe("|"), "");
    assert_eq!(closing_field_pipe("cat"), "cat");
    assert_eq!(closing_field_pipe("||one|猫|ab"), "ab");
    assert_eq!(closing_field_pipe("aa|bb"), "bb");
    assert_eq!(closing_field_pipe("ok|🐈"), "🐈");
    assert_eq!(closing_field_pipe("header\nleft|right\r\n"), "right");
    assert_eq!(closing_field_pipe("header\n\n"), "");
    assert_eq!(closing_field_pipe("猫|x\nend|a"), "a");
}

#[test]
fn submission_marker() {
    assert_eq!(task_578::TRACK_PROOF, 0x52540000 | 578);
}
