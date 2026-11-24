use task_482::*;

#[test]
fn edge_cases() {
    assert_eq!(marks_removed_at_both_ends_pipe(("hi|kit", "x")), 6);
    assert_eq!(marks_removed_at_both_ends_pipe(("|é|a", "z|w")), 4);
    assert_eq!(marks_removed_at_both_ends_pipe(("", "x")), 1);
    assert_eq!(marks_removed_at_both_ends_pipe(("|", "z|w")), 3);
    assert_eq!(marks_removed_at_both_ends_pipe(("cat", "x")), 3);
    assert_eq!(marks_removed_at_both_ends_pipe(("||one|猫|ab", "z|w")), 10);
    assert_eq!(marks_removed_at_both_ends_pipe(("aa|bb", "x")), 5);
    assert_eq!(marks_removed_at_both_ends_pipe(("ok|🐈", "z|w")), 7);
    assert_eq!(
        marks_removed_at_both_ends_pipe(("header\nleft|right\r\n", "x")),
        10
    );
    assert_eq!(marks_removed_at_both_ends_pipe(("header\n\n", "z|w")), 0);
    assert_eq!(marks_removed_at_both_ends_pipe(("猫|x\nend|a", "x")), 5);
}

#[test]
fn submission_marker() {
    assert_eq!(task_482::TRACK_PROOF, 0x52540000 | 482);
}
