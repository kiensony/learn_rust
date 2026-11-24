use task_322::*;

#[test]
fn edge_cases() {
    assert_eq!(
        marks_removed_at_both_ends_slash(String::from("hi/kit")),
        "hi/kit"
    );
    assert_eq!(
        marks_removed_at_both_ends_slash(String::from("/é/a")),
        "é/a"
    );
    assert_eq!(marks_removed_at_both_ends_slash(String::from("")), "");
    assert_eq!(marks_removed_at_both_ends_slash(String::from("/")), "");
    assert_eq!(marks_removed_at_both_ends_slash(String::from("cat")), "cat");
    assert_eq!(
        marks_removed_at_both_ends_slash(String::from("//one/猫/ab")),
        "one/猫/ab"
    );
    assert_eq!(
        marks_removed_at_both_ends_slash(String::from("aa/bb")),
        "aa/bb"
    );
    assert_eq!(
        marks_removed_at_both_ends_slash(String::from("ok/🐈")),
        "ok/🐈"
    );
    assert_eq!(
        marks_removed_at_both_ends_slash(String::from("path/first/last")),
        "first/last"
    );
    assert_eq!(
        marks_removed_at_both_ends_slash(String::from("path/path/")),
        "path"
    );
    assert_eq!(marks_removed_at_both_ends_slash(String::from("path/")), "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_322::TRACK_PROOF, 0x52540000 | 322);
}
