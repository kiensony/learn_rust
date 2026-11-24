use task_939::*;

#[test]
fn edge_cases() {
    assert_eq!(
        prefix_before_final_mark_equals("hi=kit", |s| s.chars().count() + 1),
        3
    );
    assert_eq!(prefix_before_final_mark_equals("=é=a", |s| s.len() * 2), 6);
    assert_eq!(
        prefix_before_final_mark_equals("", |s| s.chars().count() + 1),
        1
    );
    assert_eq!(prefix_before_final_mark_equals("=", |s| s.len() * 2), 0);
    assert_eq!(
        prefix_before_final_mark_equals("cat", |s| s.chars().count() + 1),
        1
    );
    assert_eq!(
        prefix_before_final_mark_equals("==one=猫=ab", |s| s.len() * 2),
        18
    );
    assert_eq!(
        prefix_before_final_mark_equals("aa=bb", |s| s.chars().count() + 1),
        3
    );
    assert_eq!(prefix_before_final_mark_equals("ok=🐈", |s| s.len() * 2), 4);
    assert_eq!(
        prefix_before_final_mark_equals("aa=bb;meta=123", |s| s.chars().count() + 1),
        3
    );
    assert_eq!(
        prefix_before_final_mark_equals(";ignore=all", |s| s.len() * 2),
        0
    );
    assert_eq!(
        prefix_before_final_mark_equals("a==b;", |s| s.chars().count() + 1),
        3
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_939::TRACK_PROOF, 0x52540000 | 939);
}
