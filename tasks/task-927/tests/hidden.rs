use task_927::*;

#[test]
fn edge_cases() {
    assert_eq!(
        marks_removed_at_both_ends_pipe("hi|kit", |s| s.chars().count() + 1),
        7
    );
    assert_eq!(marks_removed_at_both_ends_pipe("|é|a", |s| s.len() * 2), 8);
    assert_eq!(
        marks_removed_at_both_ends_pipe("", |s| s.chars().count() + 1),
        1
    );
    assert_eq!(marks_removed_at_both_ends_pipe("|", |s| s.len() * 2), 0);
    assert_eq!(
        marks_removed_at_both_ends_pipe("cat", |s| s.chars().count() + 1),
        4
    );
    assert_eq!(
        marks_removed_at_both_ends_pipe("||one|猫|ab", |s| s.len() * 2),
        20
    );
    assert_eq!(
        marks_removed_at_both_ends_pipe("aa|bb", |s| s.chars().count() + 1),
        6
    );
    assert_eq!(
        marks_removed_at_both_ends_pipe("ok|🐈", |s| s.len() * 2),
        14
    );
    assert_eq!(
        marks_removed_at_both_ends_pipe("header\nleft|right\r\n", |s| s.chars().count() + 1),
        11
    );
    assert_eq!(
        marks_removed_at_both_ends_pipe("header\n\n", |s| s.len() * 2),
        0
    );
    assert_eq!(
        marks_removed_at_both_ends_pipe("猫|x\nend|a", |s| s.chars().count() + 1),
        6
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_927::TRACK_PROOF, 0x52540000 | 927);
}
