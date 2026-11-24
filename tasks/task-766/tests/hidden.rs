use task_766::*;

#[test]
fn edge_cases() {
    assert_eq!(
        closing_marks_removed_slash(&[]),
        std::collections::BTreeMap::from([])
    );
    assert_eq!(
        closing_marks_removed_slash(&["path/1/12", "path/1/12", "path/123"]),
        std::collections::BTreeMap::from([
            (String::from("1/12"), 2usize),
            (String::from("123"), 1usize)
        ])
    );
    assert_eq!(
        closing_marks_removed_slash(&[
            "",
            "/",
            "cat",
            "//one/猫/ab",
            "aa/bb",
            "ok/🐈",
            "path/first/last",
            "path/path/",
            "path/"
        ]),
        std::collections::BTreeMap::from([
            (String::from(""), 3usize),
            (String::from("cat"), 1usize),
            (String::from("//one/猫/ab"), 1usize),
            (String::from("aa/bb"), 1usize),
            (String::from("ok/🐈"), 1usize),
            (String::from("first/last"), 1usize),
            (String::from("path"), 1usize)
        ])
    );
    assert_eq!(
        closing_marks_removed_slash(&["", "", "/", ""]),
        std::collections::BTreeMap::from([(String::from(""), 4usize)])
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_766::TRACK_PROOF, 0x52540000 | 766);
}
