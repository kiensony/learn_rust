use task_323::*;

#[test]
fn edge_cases() {
    assert_eq!(
        remainder_after_first_mark_slash(String::from("hi/kit")),
        "kit"
    );
    assert_eq!(
        remainder_after_first_mark_slash(String::from("/é/a")),
        "é/a"
    );
    assert_eq!(remainder_after_first_mark_slash(String::from("")), "");
    assert_eq!(remainder_after_first_mark_slash(String::from("/")), "");
    assert_eq!(remainder_after_first_mark_slash(String::from("cat")), "");
    assert_eq!(
        remainder_after_first_mark_slash(String::from("//one/猫/ab")),
        "/one/猫/ab"
    );
    assert_eq!(
        remainder_after_first_mark_slash(String::from("aa/bb")),
        "bb"
    );
    assert_eq!(
        remainder_after_first_mark_slash(String::from("ok/🐈")),
        "🐈"
    );
    assert_eq!(
        remainder_after_first_mark_slash(String::from("path/first/last")),
        "last"
    );
    assert_eq!(
        remainder_after_first_mark_slash(String::from("path/path/")),
        ""
    );
    assert_eq!(remainder_after_first_mark_slash(String::from("path/")), "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_323::TRACK_PROOF, 0x52540000 | 323);
}
