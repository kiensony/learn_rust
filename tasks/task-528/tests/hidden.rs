use task_528::*;

#[test]
fn edge_cases() {
    assert_eq!(leading_field_pipe("hi|kit"), 2);
    assert_eq!(leading_field_pipe("|é|a"), 0);
    assert_eq!(leading_field_pipe(""), 0);
    assert_eq!(leading_field_pipe("|"), 0);
    assert_eq!(leading_field_pipe("cat"), 3);
    assert_eq!(leading_field_pipe("||one|猫|ab"), 0);
    assert_eq!(leading_field_pipe("aa|bb"), 2);
    assert_eq!(leading_field_pipe("ok|🐈"), 2);
    assert_eq!(leading_field_pipe("header\nleft|right\r\n"), 4);
    assert_eq!(leading_field_pipe("header\n\n"), 0);
    assert_eq!(leading_field_pipe("猫|x\nend|a"), 3);
}

#[test]
fn submission_marker() {
    assert_eq!(task_528::TRACK_PROOF, 0x52540000 | 528);
}
