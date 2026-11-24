use task_1017::*;

#[test]
fn edge_cases() {
    assert_eq!(remainder_after_first_mark_slash(["hi/kit"]), [3]);
    assert_eq!(
        remainder_after_first_mark_slash([
            String::from("/é/a"),
            String::from(""),
            String::from("/"),
            String::from("cat"),
            String::from("//one/猫/ab"),
            String::from("aa/bb"),
            String::from("ok/🐈"),
            String::from("path/first/last"),
            String::from("path/path/"),
            String::from("path/")
        ]),
        [4, 0, 0, 0, 11, 2, 4, 4, 0, 0]
    );
    assert_eq!(
        remainder_after_first_mark_slash([
            "path/1/12",
            "path/123",
            "hi/kit",
            "/é/a",
            "",
            "/",
            "cat",
            "//one/猫/ab",
            "aa/bb",
            "ok/🐈",
            "path/first/last",
            "path/path/",
            "path/",
            "path/1/12"
        ]),
        [2, 0, 3, 4, 0, 0, 0, 11, 2, 4, 4, 0, 0, 2]
    );
    assert_eq!(
        remainder_after_first_mark_slash([String::from(""), String::from("/"), String::from("/")]),
        [0, 0, 0]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1017::TRACK_PROOF, 0x52540000 | 1017);
}
