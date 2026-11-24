use task_941::*;

#[test]
fn edge_cases() {
    assert_eq!(
        longest_field_with_last_tie_equals("hi=kit", |s| s.chars().count() + 1),
        4
    );
    assert_eq!(
        longest_field_with_last_tie_equals("=é=a", |s| s.len() * 2),
        4
    );
    assert_eq!(
        longest_field_with_last_tie_equals("", |s| s.chars().count() + 1),
        1
    );
    assert_eq!(longest_field_with_last_tie_equals("=", |s| s.len() * 2), 0);
    assert_eq!(
        longest_field_with_last_tie_equals("cat", |s| s.chars().count() + 1),
        4
    );
    assert_eq!(
        longest_field_with_last_tie_equals("==one=猫=ab", |s| s.len() * 2),
        6
    );
    assert_eq!(
        longest_field_with_last_tie_equals("aa=bb", |s| s.chars().count() + 1),
        3
    );
    assert_eq!(
        longest_field_with_last_tie_equals("ok=🐈", |s| s.len() * 2),
        8
    );
    assert_eq!(
        longest_field_with_last_tie_equals("aa=bb;meta=123", |s| s.chars().count() + 1),
        3
    );
    assert_eq!(
        longest_field_with_last_tie_equals(";ignore=all", |s| s.len() * 2),
        0
    );
    assert_eq!(
        longest_field_with_last_tie_equals("a==b;", |s| s.chars().count() + 1),
        2
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_941::TRACK_PROOF, 0x52540000 | 941);
}
