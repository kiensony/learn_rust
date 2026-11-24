use task_332::*;

#[test]
fn edge_cases() {
    assert_eq!(
        marks_removed_at_both_ends_pipe(String::from("hi|kit")),
        "hi|kit"
    );
    assert_eq!(marks_removed_at_both_ends_pipe(String::from("|é|a")), "é|a");
    assert_eq!(marks_removed_at_both_ends_pipe(String::from("")), "");
    assert_eq!(marks_removed_at_both_ends_pipe(String::from("|")), "");
    assert_eq!(marks_removed_at_both_ends_pipe(String::from("cat")), "cat");
    assert_eq!(
        marks_removed_at_both_ends_pipe(String::from("||one|猫|ab")),
        "one|猫|ab"
    );
    assert_eq!(
        marks_removed_at_both_ends_pipe(String::from("aa|bb")),
        "aa|bb"
    );
    assert_eq!(
        marks_removed_at_both_ends_pipe(String::from("ok|🐈")),
        "ok|🐈"
    );
    assert_eq!(
        marks_removed_at_both_ends_pipe(String::from("header\nleft|right\r\n")),
        "left|right"
    );
    assert_eq!(
        marks_removed_at_both_ends_pipe(String::from("header\n\n")),
        ""
    );
    assert_eq!(
        marks_removed_at_both_ends_pipe(String::from("猫|x\nend|a")),
        "end|a"
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_332::TRACK_PROOF, 0x52540000 | 332);
}
