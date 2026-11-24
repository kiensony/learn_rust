use task_934::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_field_equals("hi=kit", |s| s.chars().count() + 1), 4);
    assert_eq!(closing_field_equals("=é=a", |s| s.len() * 2), 2);
    assert_eq!(closing_field_equals("", |s| s.chars().count() + 1), 1);
    assert_eq!(closing_field_equals("=", |s| s.len() * 2), 0);
    assert_eq!(closing_field_equals("cat", |s| s.chars().count() + 1), 4);
    assert_eq!(closing_field_equals("==one=猫=ab", |s| s.len() * 2), 4);
    assert_eq!(closing_field_equals("aa=bb", |s| s.chars().count() + 1), 3);
    assert_eq!(closing_field_equals("ok=🐈", |s| s.len() * 2), 8);
    assert_eq!(
        closing_field_equals("aa=bb;meta=123", |s| s.chars().count() + 1),
        3
    );
    assert_eq!(closing_field_equals(";ignore=all", |s| s.len() * 2), 0);
    assert_eq!(closing_field_equals("a==b;", |s| s.chars().count() + 1), 2);
}

#[test]
fn submission_marker() {
    assert_eq!(task_934::TRACK_PROOF, 0x52540000 | 934);
}
