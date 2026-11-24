use task_1169::*;

#[test]
fn edge_cases() {
    assert_eq!(longest_field_with_last_tie_slash("hi/kit", "backup"), "kit");
    assert_eq!(longest_field_with_last_tie_slash("/é/a", "backup"), "é");
    assert_eq!(longest_field_with_last_tie_slash("", "backup"), "backup");
    assert_eq!(longest_field_with_last_tie_slash("/", "backup"), "backup");
    assert_eq!(longest_field_with_last_tie_slash("cat", "backup"), "cat");
    assert_eq!(
        longest_field_with_last_tie_slash("//one/猫/ab", "backup"),
        "猫"
    );
    assert_eq!(longest_field_with_last_tie_slash("aa/bb", "backup"), "bb");
    assert_eq!(longest_field_with_last_tie_slash("ok/🐈", "backup"), "🐈");
    assert_eq!(
        longest_field_with_last_tie_slash("path/first/last", "backup"),
        "first"
    );
    assert_eq!(
        longest_field_with_last_tie_slash("path/path/", "backup"),
        "path"
    );
    assert_eq!(
        longest_field_with_last_tie_slash("path/", "backup"),
        "backup"
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1169::TRACK_PROOF, 0x52540000 | 1169);
}
