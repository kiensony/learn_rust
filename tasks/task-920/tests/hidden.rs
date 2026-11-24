use task_920::*;

#[test]
fn edge_cases() {
    assert_eq!(
        first_nonempty_field_slash("hi/kit", |s| s.chars().count() + 1),
        3
    );
    assert_eq!(first_nonempty_field_slash("/é/a", |s| s.len() * 2), 4);
    assert_eq!(first_nonempty_field_slash("", |s| s.chars().count() + 1), 1);
    assert_eq!(first_nonempty_field_slash("/", |s| s.len() * 2), 0);
    assert_eq!(
        first_nonempty_field_slash("cat", |s| s.chars().count() + 1),
        4
    );
    assert_eq!(
        first_nonempty_field_slash("//one/猫/ab", |s| s.len() * 2),
        6
    );
    assert_eq!(
        first_nonempty_field_slash("aa/bb", |s| s.chars().count() + 1),
        3
    );
    assert_eq!(first_nonempty_field_slash("ok/🐈", |s| s.len() * 2), 4);
    assert_eq!(
        first_nonempty_field_slash("path/first/last", |s| s.chars().count() + 1),
        6
    );
    assert_eq!(first_nonempty_field_slash("path/path/", |s| s.len() * 2), 8);
    assert_eq!(
        first_nonempty_field_slash("path/", |s| s.chars().count() + 1),
        1
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_920::TRACK_PROOF, 0x52540000 | 920);
}
