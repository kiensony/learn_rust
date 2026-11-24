use task_575::*;

#[test]
fn edge_cases() {
    assert_eq!(longest_field_with_last_tie_slash("hi/kit"), "kit");
    assert_eq!(longest_field_with_last_tie_slash("/é/a"), "é");
    assert_eq!(longest_field_with_last_tie_slash(""), "");
    assert_eq!(longest_field_with_last_tie_slash("/"), "");
    assert_eq!(longest_field_with_last_tie_slash("cat"), "cat");
    assert_eq!(longest_field_with_last_tie_slash("//one/猫/ab"), "猫");
    assert_eq!(longest_field_with_last_tie_slash("aa/bb"), "bb");
    assert_eq!(longest_field_with_last_tie_slash("ok/🐈"), "🐈");
    assert_eq!(
        longest_field_with_last_tie_slash("path/first/last"),
        "first"
    );
    assert_eq!(longest_field_with_last_tie_slash("path/path/"), "path");
    assert_eq!(longest_field_with_last_tie_slash("path/"), "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_575::TRACK_PROOF, 0x52540000 | 575);
}
