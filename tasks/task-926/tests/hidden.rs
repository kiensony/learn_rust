use task_926::*;

#[test]
fn edge_cases() {
    assert_eq!(
        closing_marks_removed_pipe("hi|kit", |s| s.chars().count() + 1),
        7
    );
    assert_eq!(closing_marks_removed_pipe("|é|a", |s| s.len() * 2), 10);
    assert_eq!(closing_marks_removed_pipe("", |s| s.chars().count() + 1), 1);
    assert_eq!(closing_marks_removed_pipe("|", |s| s.len() * 2), 0);
    assert_eq!(
        closing_marks_removed_pipe("cat", |s| s.chars().count() + 1),
        4
    );
    assert_eq!(
        closing_marks_removed_pipe("||one|猫|ab", |s| s.len() * 2),
        24
    );
    assert_eq!(
        closing_marks_removed_pipe("aa|bb", |s| s.chars().count() + 1),
        6
    );
    assert_eq!(closing_marks_removed_pipe("ok|🐈", |s| s.len() * 2), 14);
    assert_eq!(
        closing_marks_removed_pipe("header\nleft|right\r\n", |s| s.chars().count() + 1),
        11
    );
    assert_eq!(closing_marks_removed_pipe("header\n\n", |s| s.len() * 2), 0);
    assert_eq!(
        closing_marks_removed_pipe("猫|x\nend|a", |s| s.chars().count() + 1),
        6
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_926::TRACK_PROOF, 0x52540000 | 926);
}
