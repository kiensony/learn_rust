use task_1190::*;

#[test]
fn edge_cases() {
    assert_eq!(
        shortest_field_with_first_tie_equals("hi=kit", "backup"),
        "hi"
    );
    assert_eq!(
        shortest_field_with_first_tie_equals("=é=a", "backup"),
        "backup"
    );
    assert_eq!(shortest_field_with_first_tie_equals("", "backup"), "backup");
    assert_eq!(
        shortest_field_with_first_tie_equals("=", "backup"),
        "backup"
    );
    assert_eq!(shortest_field_with_first_tie_equals("cat", "backup"), "cat");
    assert_eq!(
        shortest_field_with_first_tie_equals("==one=猫=ab", "backup"),
        "backup"
    );
    assert_eq!(
        shortest_field_with_first_tie_equals("aa=bb", "backup"),
        "aa"
    );
    assert_eq!(
        shortest_field_with_first_tie_equals("ok=🐈", "backup"),
        "ok"
    );
    assert_eq!(
        shortest_field_with_first_tie_equals("aa=bb;meta=123", "backup"),
        "aa"
    );
    assert_eq!(
        shortest_field_with_first_tie_equals(";ignore=all", "backup"),
        "backup"
    );
    assert_eq!(
        shortest_field_with_first_tie_equals("a==b;", "backup"),
        "backup"
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1190::TRACK_PROOF, 0x52540000 | 1190);
}
