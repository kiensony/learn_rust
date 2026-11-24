use task_1014::*;

#[test]
fn edge_cases() {
    assert_eq!(leading_marks_removed_slash(["hi/kit"]), [6]);
    assert_eq!(
        leading_marks_removed_slash([
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
        [4, 0, 0, 3, 10, 5, 7, 10, 5, 0]
    );
    assert_eq!(
        leading_marks_removed_slash([
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
        [4, 3, 6, 4, 0, 0, 3, 10, 5, 7, 10, 5, 0, 4]
    );
    assert_eq!(
        leading_marks_removed_slash([String::from(""), String::from("/"), String::from("/")]),
        [0, 0, 0]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1014::TRACK_PROOF, 0x52540000 | 1014);
}
