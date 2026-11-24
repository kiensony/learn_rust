use task_479::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_field_pipe(("header\n猫|123", "z|w")), 3);
    assert_eq!(closing_field_pipe(("hi|kit", "x")), 3);
    assert_eq!(closing_field_pipe(("", "x")), 1);
    assert_eq!(closing_field_pipe(("|", "z|w")), 1);
    assert_eq!(closing_field_pipe(("cat", "x")), 3);
    assert_eq!(closing_field_pipe(("||one|猫|ab", "z|w")), 2);
    assert_eq!(closing_field_pipe(("aa|bb", "x")), 2);
    assert_eq!(closing_field_pipe(("ok|🐈", "z|w")), 4);
    assert_eq!(closing_field_pipe(("header\nleft|right\r\n", "x")), 5);
    assert_eq!(closing_field_pipe(("header\n\n", "z|w")), 0);
    assert_eq!(closing_field_pipe(("猫|x\nend|a", "x")), 1);
}

#[test]
fn submission_marker() {
    assert_eq!(task_479::TRACK_PROOF, 0x52540000 | 479);
}
