use task_529::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_field_pipe("header\n猫|123"), 3);
    assert_eq!(closing_field_pipe("hi|kit"), 3);
    assert_eq!(closing_field_pipe(""), 0);
    assert_eq!(closing_field_pipe("|"), 0);
    assert_eq!(closing_field_pipe("cat"), 3);
    assert_eq!(closing_field_pipe("||one|猫|ab"), 2);
    assert_eq!(closing_field_pipe("aa|bb"), 2);
    assert_eq!(closing_field_pipe("ok|🐈"), 1);
    assert_eq!(closing_field_pipe("header\nleft|right\r\n"), 5);
    assert_eq!(closing_field_pipe("header\n\n"), 0);
    assert_eq!(closing_field_pipe("猫|x\nend|a"), 1);
}

#[test]
fn submission_marker() {
    assert_eq!(task_529::TRACK_PROOF, 0x52540000 | 529);
}
