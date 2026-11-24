use task_321::*;

#[test]
fn edge_cases() {
    assert_eq!(
        closing_marks_removed_slash(String::from("hi/kit")),
        "hi/kit"
    );
    assert_eq!(closing_marks_removed_slash(String::from("/é/a")), "/é/a");
    assert_eq!(closing_marks_removed_slash(String::from("")), "");
    assert_eq!(closing_marks_removed_slash(String::from("/")), "");
    assert_eq!(closing_marks_removed_slash(String::from("cat")), "cat");
    assert_eq!(
        closing_marks_removed_slash(String::from("//one/猫/ab")),
        "//one/猫/ab"
    );
    assert_eq!(closing_marks_removed_slash(String::from("aa/bb")), "aa/bb");
    assert_eq!(closing_marks_removed_slash(String::from("ok/🐈")), "ok/🐈");
    assert_eq!(
        closing_marks_removed_slash(String::from("path/first/last")),
        "first/last"
    );
    assert_eq!(
        closing_marks_removed_slash(String::from("path/path/")),
        "path"
    );
    assert_eq!(closing_marks_removed_slash(String::from("path/")), "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_321::TRACK_PROOF, 0x52540000 | 321);
}
