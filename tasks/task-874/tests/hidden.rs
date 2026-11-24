use task_874::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_field_pipe("header\n猫|123"), 5);
    assert_eq!(closing_field_pipe("hi|kit"), 5);
    assert_eq!(closing_field_pipe(""), 2);
    assert_eq!(closing_field_pipe("|"), 2);
    assert_eq!(closing_field_pipe("cat"), 5);
    assert_eq!(closing_field_pipe("||one|猫|ab"), 4);
    assert_eq!(closing_field_pipe("aa|bb"), 4);
    assert_eq!(closing_field_pipe("ok|🐈"), 6);
    assert_eq!(closing_field_pipe("header\nleft|right\r\n"), 7);
    assert_eq!(closing_field_pipe("header\n\n"), 2);
    assert_eq!(closing_field_pipe("猫|x\nend|a"), 3);
}

#[test]
fn submission_marker() {
    assert_eq!(task_874::TRACK_PROOF, 0x52540000 | 874);
}
