use task_485::*;

#[test]
fn edge_cases() {
    assert_eq!(first_nonempty_field_pipe(("header\n猫|123", "z|w")), 3);
    assert_eq!(first_nonempty_field_pipe(("|é|a", "z|w")), 2);
    assert_eq!(first_nonempty_field_pipe(("", "x")), 1);
    assert_eq!(first_nonempty_field_pipe(("|", "z|w")), 1);
    assert_eq!(first_nonempty_field_pipe(("cat", "x")), 3);
    assert_eq!(first_nonempty_field_pipe(("||one|猫|ab", "z|w")), 3);
    assert_eq!(first_nonempty_field_pipe(("aa|bb", "x")), 2);
    assert_eq!(first_nonempty_field_pipe(("ok|🐈", "z|w")), 2);
    assert_eq!(
        first_nonempty_field_pipe(("header\nleft|right\r\n", "x")),
        4
    );
    assert_eq!(first_nonempty_field_pipe(("header\n\n", "z|w")), 0);
    assert_eq!(first_nonempty_field_pipe(("猫|x\nend|a", "x")), 3);
}

#[test]
fn submission_marker() {
    assert_eq!(task_485::TRACK_PROOF, 0x52540000 | 485);
}
