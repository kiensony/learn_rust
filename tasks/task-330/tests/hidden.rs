use task_330::*;

#[test]
fn edge_cases() {
    assert_eq!(leading_marks_removed_pipe(String::from("hi|kit")), "hi|kit");
    assert_eq!(leading_marks_removed_pipe(String::from("|é|a")), "é|a");
    assert_eq!(leading_marks_removed_pipe(String::from("")), "");
    assert_eq!(leading_marks_removed_pipe(String::from("|")), "");
    assert_eq!(leading_marks_removed_pipe(String::from("cat")), "cat");
    assert_eq!(
        leading_marks_removed_pipe(String::from("||one|猫|ab")),
        "one|猫|ab"
    );
    assert_eq!(leading_marks_removed_pipe(String::from("aa|bb")), "aa|bb");
    assert_eq!(leading_marks_removed_pipe(String::from("ok|🐈")), "ok|🐈");
    assert_eq!(
        leading_marks_removed_pipe(String::from("header\nleft|right\r\n")),
        "left|right"
    );
    assert_eq!(leading_marks_removed_pipe(String::from("header\n\n")), "");
    assert_eq!(
        leading_marks_removed_pipe(String::from("猫|x\nend|a")),
        "end|a"
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_330::TRACK_PROOF, 0x52540000 | 330);
}
