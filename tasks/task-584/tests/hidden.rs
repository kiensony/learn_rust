use task_584::*;

#[test]
fn edge_cases() {
    assert_eq!(first_nonempty_field_pipe("hi|kit"), "hi");
    assert_eq!(first_nonempty_field_pipe("|é|a"), "é");
    assert_eq!(first_nonempty_field_pipe(""), "");
    assert_eq!(first_nonempty_field_pipe("|"), "");
    assert_eq!(first_nonempty_field_pipe("cat"), "cat");
    assert_eq!(first_nonempty_field_pipe("||one|猫|ab"), "one");
    assert_eq!(first_nonempty_field_pipe("aa|bb"), "aa");
    assert_eq!(first_nonempty_field_pipe("ok|🐈"), "ok");
    assert_eq!(first_nonempty_field_pipe("header\nleft|right\r\n"), "left");
    assert_eq!(first_nonempty_field_pipe("header\n\n"), "");
    assert_eq!(first_nonempty_field_pipe("猫|x\nend|a"), "end");
}

#[test]
fn submission_marker() {
    assert_eq!(task_584::TRACK_PROOF, 0x52540000 | 584);
}
