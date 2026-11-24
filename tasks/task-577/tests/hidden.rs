use task_577::*;

#[test]
fn edge_cases() {
    assert_eq!(leading_field_pipe("hi|kit"), "hi");
    assert_eq!(leading_field_pipe("|é|a"), "");
    assert_eq!(leading_field_pipe(""), "");
    assert_eq!(leading_field_pipe("|"), "");
    assert_eq!(leading_field_pipe("cat"), "cat");
    assert_eq!(leading_field_pipe("||one|猫|ab"), "");
    assert_eq!(leading_field_pipe("aa|bb"), "aa");
    assert_eq!(leading_field_pipe("ok|🐈"), "ok");
    assert_eq!(leading_field_pipe("header\nleft|right\r\n"), "left");
    assert_eq!(leading_field_pipe("header\n\n"), "");
    assert_eq!(leading_field_pipe("猫|x\nend|a"), "end");
}

#[test]
fn submission_marker() {
    assert_eq!(task_577::TRACK_PROOF, 0x52540000 | 577);
}
