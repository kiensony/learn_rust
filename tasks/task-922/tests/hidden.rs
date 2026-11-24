use task_922::*;

#[test]
fn edge_cases() {
    assert_eq!(
        shortest_field_with_first_tie_slash("hi/kit", |s| s.chars().count() + 1),
        3
    );
    assert_eq!(
        shortest_field_with_first_tie_slash("/é/a", |s| s.len() * 2),
        0
    );
    assert_eq!(
        shortest_field_with_first_tie_slash("", |s| s.chars().count() + 1),
        1
    );
    assert_eq!(shortest_field_with_first_tie_slash("/", |s| s.len() * 2), 0);
    assert_eq!(
        shortest_field_with_first_tie_slash("cat", |s| s.chars().count() + 1),
        4
    );
    assert_eq!(
        shortest_field_with_first_tie_slash("//one/猫/ab", |s| s.len() * 2),
        0
    );
    assert_eq!(
        shortest_field_with_first_tie_slash("aa/bb", |s| s.chars().count() + 1),
        3
    );
    assert_eq!(
        shortest_field_with_first_tie_slash("ok/🐈", |s| s.len() * 2),
        4
    );
    assert_eq!(
        shortest_field_with_first_tie_slash("path/first/last", |s| s.chars().count() + 1),
        5
    );
    assert_eq!(
        shortest_field_with_first_tie_slash("path/path/", |s| s.len() * 2),
        0
    );
    assert_eq!(
        shortest_field_with_first_tie_slash("path/", |s| s.chars().count() + 1),
        1
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_922::TRACK_PROOF, 0x52540000 | 922);
}
