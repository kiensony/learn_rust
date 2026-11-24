use task_333::*;

#[test]
fn edge_cases() {
    assert_eq!(
        remainder_after_first_mark_pipe(String::from("hi|kit")),
        "kit"
    );
    assert_eq!(remainder_after_first_mark_pipe(String::from("|é|a")), "é|a");
    assert_eq!(remainder_after_first_mark_pipe(String::from("")), "");
    assert_eq!(remainder_after_first_mark_pipe(String::from("|")), "");
    assert_eq!(remainder_after_first_mark_pipe(String::from("cat")), "");
    assert_eq!(
        remainder_after_first_mark_pipe(String::from("||one|猫|ab")),
        "|one|猫|ab"
    );
    assert_eq!(remainder_after_first_mark_pipe(String::from("aa|bb")), "bb");
    assert_eq!(remainder_after_first_mark_pipe(String::from("ok|🐈")), "🐈");
    assert_eq!(
        remainder_after_first_mark_pipe(String::from("header\nleft|right\r\n")),
        "right"
    );
    assert_eq!(
        remainder_after_first_mark_pipe(String::from("header\n\n")),
        ""
    );
    assert_eq!(
        remainder_after_first_mark_pipe(String::from("猫|x\nend|a")),
        "a"
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_333::TRACK_PROOF, 0x52540000 | 333);
}
