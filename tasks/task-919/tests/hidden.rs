use task_919::*;

#[test]
fn edge_cases() {
    assert_eq!(
        prefix_before_final_mark_slash("hi/kit", |s| s.chars().count() + 1),
        3
    );
    assert_eq!(prefix_before_final_mark_slash("/é/a", |s| s.len() * 2), 6);
    assert_eq!(
        prefix_before_final_mark_slash("", |s| s.chars().count() + 1),
        1
    );
    assert_eq!(prefix_before_final_mark_slash("/", |s| s.len() * 2), 0);
    assert_eq!(
        prefix_before_final_mark_slash("cat", |s| s.chars().count() + 1),
        1
    );
    assert_eq!(
        prefix_before_final_mark_slash("//one/猫/ab", |s| s.len() * 2),
        18
    );
    assert_eq!(
        prefix_before_final_mark_slash("aa/bb", |s| s.chars().count() + 1),
        3
    );
    assert_eq!(prefix_before_final_mark_slash("ok/🐈", |s| s.len() * 2), 4);
    assert_eq!(
        prefix_before_final_mark_slash("path/first/last", |s| s.chars().count() + 1),
        6
    );
    assert_eq!(
        prefix_before_final_mark_slash("path/path/", |s| s.len() * 2),
        8
    );
    assert_eq!(
        prefix_before_final_mark_slash("path/", |s| s.chars().count() + 1),
        1
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_919::TRACK_PROOF, 0x52540000 | 919);
}
