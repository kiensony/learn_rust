use task_870::*;

#[test]
fn edge_cases() {
    assert_eq!(first_nonempty_field_slash("hi/kit"), 4);
    assert_eq!(first_nonempty_field_slash("/é/a"), 4);
    assert_eq!(first_nonempty_field_slash(""), 2);
    assert_eq!(first_nonempty_field_slash("/"), 2);
    assert_eq!(first_nonempty_field_slash("cat"), 5);
    assert_eq!(first_nonempty_field_slash("//one/猫/ab"), 5);
    assert_eq!(first_nonempty_field_slash("aa/bb"), 4);
    assert_eq!(first_nonempty_field_slash("ok/🐈"), 4);
    assert_eq!(first_nonempty_field_slash("path/first/last"), 7);
    assert_eq!(first_nonempty_field_slash("path/path/"), 6);
    assert_eq!(first_nonempty_field_slash("path/"), 2);
}

#[test]
fn submission_marker() {
    assert_eq!(task_870::TRACK_PROOF, 0x52540000 | 870);
}
