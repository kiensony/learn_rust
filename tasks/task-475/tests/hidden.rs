use task_475::*;

#[test]
fn edge_cases() {
    assert_eq!(first_nonempty_field_slash(("hi/kit", "x")), 2);
    assert_eq!(first_nonempty_field_slash(("/é/a", "z/w")), 2);
    assert_eq!(first_nonempty_field_slash(("", "x")), 1);
    assert_eq!(first_nonempty_field_slash(("/", "z/w")), 1);
    assert_eq!(first_nonempty_field_slash(("cat", "x")), 3);
    assert_eq!(first_nonempty_field_slash(("//one/猫/ab", "z/w")), 3);
    assert_eq!(first_nonempty_field_slash(("aa/bb", "x")), 2);
    assert_eq!(first_nonempty_field_slash(("ok/🐈", "z/w")), 2);
    assert_eq!(first_nonempty_field_slash(("path/first/last", "x")), 5);
    assert_eq!(first_nonempty_field_slash(("path/path/", "z/w")), 4);
    assert_eq!(first_nonempty_field_slash(("path/", "x")), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_475::TRACK_PROOF, 0x52540000 | 475);
}
