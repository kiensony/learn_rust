use task_942::*;

#[test]
fn edge_cases() {
    assert_eq!(
        shortest_field_with_first_tie_equals("hi=kit", |s| s.chars().count() + 1),
        3
    );
    assert_eq!(
        shortest_field_with_first_tie_equals("=é=a", |s| s.len() * 2),
        0
    );
    assert_eq!(
        shortest_field_with_first_tie_equals("", |s| s.chars().count() + 1),
        1
    );
    assert_eq!(
        shortest_field_with_first_tie_equals("=", |s| s.len() * 2),
        0
    );
    assert_eq!(
        shortest_field_with_first_tie_equals("cat", |s| s.chars().count() + 1),
        4
    );
    assert_eq!(
        shortest_field_with_first_tie_equals("==one=猫=ab", |s| s.len() * 2),
        0
    );
    assert_eq!(
        shortest_field_with_first_tie_equals("aa=bb", |s| s.chars().count() + 1),
        3
    );
    assert_eq!(
        shortest_field_with_first_tie_equals("ok=🐈", |s| s.len() * 2),
        4
    );
    assert_eq!(
        shortest_field_with_first_tie_equals("aa=bb;meta=123", |s| s.chars().count() + 1),
        3
    );
    assert_eq!(
        shortest_field_with_first_tie_equals(";ignore=all", |s| s.len() * 2),
        0
    );
    assert_eq!(
        shortest_field_with_first_tie_equals("a==b;", |s| s.chars().count() + 1),
        1
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_942::TRACK_PROOF, 0x52540000 | 942);
}
