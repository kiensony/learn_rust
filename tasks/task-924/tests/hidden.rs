use task_924::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_field_pipe("hi|kit", |s| s.chars().count() + 1), 4);
    assert_eq!(closing_field_pipe("|é|a", |s| s.len() * 2), 2);
    assert_eq!(closing_field_pipe("", |s| s.chars().count() + 1), 1);
    assert_eq!(closing_field_pipe("|", |s| s.len() * 2), 0);
    assert_eq!(closing_field_pipe("cat", |s| s.chars().count() + 1), 4);
    assert_eq!(closing_field_pipe("||one|猫|ab", |s| s.len() * 2), 4);
    assert_eq!(closing_field_pipe("aa|bb", |s| s.chars().count() + 1), 3);
    assert_eq!(closing_field_pipe("ok|🐈", |s| s.len() * 2), 8);
    assert_eq!(
        closing_field_pipe("header\nleft|right\r\n", |s| s.chars().count() + 1),
        6
    );
    assert_eq!(closing_field_pipe("header\n\n", |s| s.len() * 2), 0);
    assert_eq!(
        closing_field_pipe("猫|x\nend|a", |s| s.chars().count() + 1),
        2
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_924::TRACK_PROOF, 0x52540000 | 924);
}
