use task_329::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_field_pipe(String::from("header\n猫|123")), "123");
    assert_eq!(closing_field_pipe(String::from("|é|a")), "a");
    assert_eq!(closing_field_pipe(String::from("")), "");
    assert_eq!(closing_field_pipe(String::from("|")), "");
    assert_eq!(closing_field_pipe(String::from("cat")), "cat");
    assert_eq!(closing_field_pipe(String::from("||one|猫|ab")), "ab");
    assert_eq!(closing_field_pipe(String::from("aa|bb")), "bb");
    assert_eq!(closing_field_pipe(String::from("ok|🐈")), "🐈");
    assert_eq!(
        closing_field_pipe(String::from("header\nleft|right\r\n")),
        "right"
    );
    assert_eq!(closing_field_pipe(String::from("header\n\n")), "");
    assert_eq!(closing_field_pipe(String::from("猫|x\nend|a")), "a");
}

#[test]
fn submission_marker() {
    assert_eq!(task_329::TRACK_PROOF, 0x52540000 | 329);
}
