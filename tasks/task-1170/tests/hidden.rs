use task_1170::*;

#[test]
fn edge_cases() {
    assert_eq!(
        shortest_field_with_first_tie_slash("hi/kit", "backup"),
        "hi"
    );
    assert_eq!(
        shortest_field_with_first_tie_slash("/é/a", "backup"),
        "backup"
    );
    assert_eq!(shortest_field_with_first_tie_slash("", "backup"), "backup");
    assert_eq!(shortest_field_with_first_tie_slash("/", "backup"), "backup");
    assert_eq!(shortest_field_with_first_tie_slash("cat", "backup"), "cat");
    assert_eq!(
        shortest_field_with_first_tie_slash("//one/猫/ab", "backup"),
        "backup"
    );
    assert_eq!(shortest_field_with_first_tie_slash("aa/bb", "backup"), "aa");
    assert_eq!(shortest_field_with_first_tie_slash("ok/🐈", "backup"), "ok");
    assert_eq!(
        shortest_field_with_first_tie_slash("path/first/last", "backup"),
        "last"
    );
    assert_eq!(
        shortest_field_with_first_tie_slash("path/path/", "backup"),
        "backup"
    );
    assert_eq!(
        shortest_field_with_first_tie_slash("path/", "backup"),
        "backup"
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1170::TRACK_PROOF, 0x52540000 | 1170);
}
