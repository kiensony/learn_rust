use task_917::*;

#[test]
fn edge_cases() {
    assert_eq!(
        marks_removed_at_both_ends_slash("hi/kit", |s| s.chars().count() + 1),
        7
    );
    assert_eq!(marks_removed_at_both_ends_slash("/é/a", |s| s.len() * 2), 8);
    assert_eq!(
        marks_removed_at_both_ends_slash("", |s| s.chars().count() + 1),
        1
    );
    assert_eq!(marks_removed_at_both_ends_slash("/", |s| s.len() * 2), 0);
    assert_eq!(
        marks_removed_at_both_ends_slash("cat", |s| s.chars().count() + 1),
        4
    );
    assert_eq!(
        marks_removed_at_both_ends_slash("//one/猫/ab", |s| s.len() * 2),
        20
    );
    assert_eq!(
        marks_removed_at_both_ends_slash("aa/bb", |s| s.chars().count() + 1),
        6
    );
    assert_eq!(
        marks_removed_at_both_ends_slash("ok/🐈", |s| s.len() * 2),
        14
    );
    assert_eq!(
        marks_removed_at_both_ends_slash("path/first/last", |s| s.chars().count() + 1),
        11
    );
    assert_eq!(
        marks_removed_at_both_ends_slash("path/path/", |s| s.len() * 2),
        8
    );
    assert_eq!(
        marks_removed_at_both_ends_slash("path/", |s| s.chars().count() + 1),
        1
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_917::TRACK_PROOF, 0x52540000 | 917);
}
