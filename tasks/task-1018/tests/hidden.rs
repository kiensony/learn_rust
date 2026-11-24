use task_1018::*;

#[test]
fn edge_cases() {
    assert_eq!(prefix_before_final_mark_slash(["hi/kit"]), [2]);
    assert_eq!(
        prefix_before_final_mark_slash([
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
        [3, 0, 0, 0, 9, 2, 2, 5, 4, 0]
    );
    assert_eq!(
        prefix_before_final_mark_slash([
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
        [1, 0, 2, 3, 0, 0, 0, 9, 2, 2, 5, 4, 0, 1]
    );
    assert_eq!(
        prefix_before_final_mark_slash([String::from(""), String::from("/"), String::from("/")]),
        [0, 0, 0]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1018::TRACK_PROOF, 0x52540000 | 1018);
}
