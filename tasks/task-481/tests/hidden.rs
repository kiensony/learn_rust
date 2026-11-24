use task_481::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_marks_removed_pipe(("hi|kit", "x")), 6);
    assert_eq!(closing_marks_removed_pipe(("|é|a", "z|w")), 5);
    assert_eq!(closing_marks_removed_pipe(("", "x")), 1);
    assert_eq!(closing_marks_removed_pipe(("|", "z|w")), 3);
    assert_eq!(closing_marks_removed_pipe(("cat", "x")), 3);
    assert_eq!(closing_marks_removed_pipe(("||one|猫|ab", "z|w")), 12);
    assert_eq!(closing_marks_removed_pipe(("aa|bb", "x")), 5);
    assert_eq!(closing_marks_removed_pipe(("ok|🐈", "z|w")), 7);
    assert_eq!(
        closing_marks_removed_pipe(("header\nleft|right\r\n", "x")),
        10
    );
    assert_eq!(closing_marks_removed_pipe(("header\n\n", "z|w")), 0);
    assert_eq!(closing_marks_removed_pipe(("猫|x\nend|a", "x")), 5);
}

#[test]
fn submission_marker() {
    assert_eq!(task_481::TRACK_PROOF, 0x52540000 | 481);
}
