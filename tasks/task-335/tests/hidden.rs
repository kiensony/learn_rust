use task_335::*;

#[test]
fn edge_cases() {
    assert_eq!(first_nonempty_field_pipe(String::from("hi|kit")), "hi");
    assert_eq!(first_nonempty_field_pipe(String::from("|é|a")), "é");
    assert_eq!(first_nonempty_field_pipe(String::from("")), "");
    assert_eq!(first_nonempty_field_pipe(String::from("|")), "");
    assert_eq!(first_nonempty_field_pipe(String::from("cat")), "cat");
    assert_eq!(
        first_nonempty_field_pipe(String::from("||one|猫|ab")),
        "one"
    );
    assert_eq!(first_nonempty_field_pipe(String::from("aa|bb")), "aa");
    assert_eq!(first_nonempty_field_pipe(String::from("ok|🐈")), "ok");
    assert_eq!(
        first_nonempty_field_pipe(String::from("header\nleft|right\r\n")),
        "left"
    );
    assert_eq!(first_nonempty_field_pipe(String::from("header\n\n")), "");
    assert_eq!(
        first_nonempty_field_pipe(String::from("猫|x\nend|a")),
        "end"
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_335::TRACK_PROOF, 0x52540000 | 335);
}
