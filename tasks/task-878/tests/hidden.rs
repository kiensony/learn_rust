use task_878::*;

#[test]
fn edge_cases() {
    assert_eq!(remainder_after_first_mark_pipe("hi|kit"), 5);
    assert_eq!(remainder_after_first_mark_pipe("|é|a"), 6);
    assert_eq!(remainder_after_first_mark_pipe(""), 2);
    assert_eq!(remainder_after_first_mark_pipe("|"), 2);
    assert_eq!(remainder_after_first_mark_pipe("cat"), 2);
    assert_eq!(remainder_after_first_mark_pipe("||one|猫|ab"), 13);
    assert_eq!(remainder_after_first_mark_pipe("aa|bb"), 4);
    assert_eq!(remainder_after_first_mark_pipe("ok|🐈"), 6);
    assert_eq!(remainder_after_first_mark_pipe("header\nleft|right\r\n"), 7);
    assert_eq!(remainder_after_first_mark_pipe("header\n\n"), 2);
    assert_eq!(remainder_after_first_mark_pipe("猫|x\nend|a"), 3);
}

#[test]
fn submission_marker() {
    assert_eq!(task_878::TRACK_PROOF, 0x52540000 | 878);
}
