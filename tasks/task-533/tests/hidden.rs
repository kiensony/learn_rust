use task_533::*;

#[test]
fn edge_cases() {
    assert_eq!(remainder_after_first_mark_pipe("hi|kit"), 3);
    assert_eq!(remainder_after_first_mark_pipe("|é|a"), 3);
    assert_eq!(remainder_after_first_mark_pipe(""), 0);
    assert_eq!(remainder_after_first_mark_pipe("|"), 0);
    assert_eq!(remainder_after_first_mark_pipe("cat"), 0);
    assert_eq!(remainder_after_first_mark_pipe("||one|猫|ab"), 9);
    assert_eq!(remainder_after_first_mark_pipe("aa|bb"), 2);
    assert_eq!(remainder_after_first_mark_pipe("ok|🐈"), 1);
    assert_eq!(remainder_after_first_mark_pipe("header\nleft|right\r\n"), 5);
    assert_eq!(remainder_after_first_mark_pipe("header\n\n"), 0);
    assert_eq!(remainder_after_first_mark_pipe("猫|x\nend|a"), 1);
}

#[test]
fn submission_marker() {
    assert_eq!(task_533::TRACK_PROOF, 0x52540000 | 533);
}
