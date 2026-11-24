use task_937::*;

#[test]
fn edge_cases() {
    assert_eq!(
        marks_removed_at_both_ends_equals("hi=kit", |s| s.chars().count() + 1),
        7
    );
    assert_eq!(
        marks_removed_at_both_ends_equals("=é=a", |s| s.len() * 2),
        8
    );
    assert_eq!(
        marks_removed_at_both_ends_equals("", |s| s.chars().count() + 1),
        1
    );
    assert_eq!(marks_removed_at_both_ends_equals("=", |s| s.len() * 2), 0);
    assert_eq!(
        marks_removed_at_both_ends_equals("cat", |s| s.chars().count() + 1),
        4
    );
    assert_eq!(
        marks_removed_at_both_ends_equals("==one=猫=ab", |s| s.len() * 2),
        20
    );
    assert_eq!(
        marks_removed_at_both_ends_equals("aa=bb", |s| s.chars().count() + 1),
        6
    );
    assert_eq!(
        marks_removed_at_both_ends_equals("ok=🐈", |s| s.len() * 2),
        14
    );
    assert_eq!(
        marks_removed_at_both_ends_equals("aa=bb;meta=123", |s| s.chars().count() + 1),
        6
    );
    assert_eq!(
        marks_removed_at_both_ends_equals(";ignore=all", |s| s.len() * 2),
        0
    );
    assert_eq!(
        marks_removed_at_both_ends_equals("a==b;", |s| s.chars().count() + 1),
        5
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_937::TRACK_PROOF, 0x52540000 | 937);
}
