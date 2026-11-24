use task_1021::*;

#[test]
fn edge_cases() {
    assert_eq!(shortest_field_with_first_tie_slash(["hi/kit"]), [2]);
    assert_eq!(
        shortest_field_with_first_tie_slash([
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
        [0, 0, 0, 3, 0, 2, 2, 4, 0, 0]
    );
    assert_eq!(
        shortest_field_with_first_tie_slash([
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
        [1, 3, 2, 0, 0, 0, 3, 0, 2, 2, 4, 0, 0, 1]
    );
    assert_eq!(
        shortest_field_with_first_tie_slash([
            String::from(""),
            String::from("/"),
            String::from("/")
        ]),
        [0, 0, 0]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1021::TRACK_PROOF, 0x52540000 | 1021);
}
