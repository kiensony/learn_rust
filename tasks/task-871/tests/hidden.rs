use task_871::*;

#[test]
fn edge_cases() {
    assert_eq!(longest_field_with_last_tie_slash("hi/kit"), 5);
    assert_eq!(longest_field_with_last_tie_slash("/é/a"), 4);
    assert_eq!(longest_field_with_last_tie_slash(""), 2);
    assert_eq!(longest_field_with_last_tie_slash("/"), 2);
    assert_eq!(longest_field_with_last_tie_slash("cat"), 5);
    assert_eq!(longest_field_with_last_tie_slash("//one/猫/ab"), 5);
    assert_eq!(longest_field_with_last_tie_slash("aa/bb"), 4);
    assert_eq!(longest_field_with_last_tie_slash("ok/🐈"), 6);
    assert_eq!(longest_field_with_last_tie_slash("path/first/last"), 7);
    assert_eq!(longest_field_with_last_tie_slash("path/path/"), 6);
    assert_eq!(longest_field_with_last_tie_slash("path/"), 2);
}

#[test]
fn submission_marker() {
    assert_eq!(task_871::TRACK_PROOF, 0x52540000 | 871);
}
