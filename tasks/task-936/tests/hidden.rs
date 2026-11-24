use task_936::*;

#[test]
fn edge_cases() {
    assert_eq!(
        closing_marks_removed_equals("hi=kit", |s| s.chars().count() + 1),
        7
    );
    assert_eq!(closing_marks_removed_equals("=é=a", |s| s.len() * 2), 10);
    assert_eq!(
        closing_marks_removed_equals("", |s| s.chars().count() + 1),
        1
    );
    assert_eq!(closing_marks_removed_equals("=", |s| s.len() * 2), 0);
    assert_eq!(
        closing_marks_removed_equals("cat", |s| s.chars().count() + 1),
        4
    );
    assert_eq!(
        closing_marks_removed_equals("==one=猫=ab", |s| s.len() * 2),
        24
    );
    assert_eq!(
        closing_marks_removed_equals("aa=bb", |s| s.chars().count() + 1),
        6
    );
    assert_eq!(closing_marks_removed_equals("ok=🐈", |s| s.len() * 2), 14);
    assert_eq!(
        closing_marks_removed_equals("aa=bb;meta=123", |s| s.chars().count() + 1),
        6
    );
    assert_eq!(
        closing_marks_removed_equals(";ignore=all", |s| s.len() * 2),
        0
    );
    assert_eq!(
        closing_marks_removed_equals("a==b;", |s| s.chars().count() + 1),
        5
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_936::TRACK_PROOF, 0x52540000 | 936);
}
