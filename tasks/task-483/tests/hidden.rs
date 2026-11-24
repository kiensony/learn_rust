use task_483::*;

#[test]
fn edge_cases() {
    assert_eq!(remainder_after_first_mark_pipe(("hi|kit", "x")), 3);
    assert_eq!(remainder_after_first_mark_pipe(("|é|a", "z|w")), 4);
    assert_eq!(remainder_after_first_mark_pipe(("", "x")), 0);
    assert_eq!(remainder_after_first_mark_pipe(("|", "z|w")), 1);
    assert_eq!(remainder_after_first_mark_pipe(("cat", "x")), 0);
    assert_eq!(remainder_after_first_mark_pipe(("||one|猫|ab", "z|w")), 11);
    assert_eq!(remainder_after_first_mark_pipe(("aa|bb", "x")), 2);
    assert_eq!(remainder_after_first_mark_pipe(("ok|🐈", "z|w")), 4);
    assert_eq!(
        remainder_after_first_mark_pipe(("header\nleft|right\r\n", "x")),
        5
    );
    assert_eq!(remainder_after_first_mark_pipe(("header\n\n", "z|w")), 0);
    assert_eq!(remainder_after_first_mark_pipe(("猫|x\nend|a", "x")), 1);
}

#[test]
fn submission_marker() {
    assert_eq!(task_483::TRACK_PROOF, 0x52540000 | 483);
}
