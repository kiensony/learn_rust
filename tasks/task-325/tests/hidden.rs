use task_325::*;

#[test]
fn edge_cases() {
    assert_eq!(first_nonempty_field_slash(String::from("hi/kit")), "hi");
    assert_eq!(first_nonempty_field_slash(String::from("/é/a")), "é");
    assert_eq!(first_nonempty_field_slash(String::from("")), "");
    assert_eq!(first_nonempty_field_slash(String::from("/")), "");
    assert_eq!(first_nonempty_field_slash(String::from("cat")), "cat");
    assert_eq!(
        first_nonempty_field_slash(String::from("//one/猫/ab")),
        "one"
    );
    assert_eq!(first_nonempty_field_slash(String::from("aa/bb")), "aa");
    assert_eq!(first_nonempty_field_slash(String::from("ok/🐈")), "ok");
    assert_eq!(
        first_nonempty_field_slash(String::from("path/first/last")),
        "first"
    );
    assert_eq!(
        first_nonempty_field_slash(String::from("path/path/")),
        "path"
    );
    assert_eq!(first_nonempty_field_slash(String::from("path/")), "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_325::TRACK_PROOF, 0x52540000 | 325);
}
