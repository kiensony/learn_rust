use task_582::*;

#[test]
fn edge_cases() {
    assert_eq!(remainder_after_first_mark_pipe("hi|kit"), "kit");
    assert_eq!(remainder_after_first_mark_pipe("|é|a"), "é|a");
    assert_eq!(remainder_after_first_mark_pipe(""), "");
    assert_eq!(remainder_after_first_mark_pipe("|"), "");
    assert_eq!(remainder_after_first_mark_pipe("cat"), "");
    assert_eq!(remainder_after_first_mark_pipe("||one|猫|ab"), "|one|猫|ab");
    assert_eq!(remainder_after_first_mark_pipe("aa|bb"), "bb");
    assert_eq!(remainder_after_first_mark_pipe("ok|🐈"), "🐈");
    assert_eq!(
        remainder_after_first_mark_pipe("header\nleft|right\r\n"),
        "right"
    );
    assert_eq!(remainder_after_first_mark_pipe("header\n\n"), "");
    assert_eq!(remainder_after_first_mark_pipe("猫|x\nend|a"), "a");
}

#[test]
fn submission_marker() {
    assert_eq!(task_582::TRACK_PROOF, 0x52540000 | 582);
}
