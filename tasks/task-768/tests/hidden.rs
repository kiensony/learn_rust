use task_768::*;

#[test]
fn edge_cases() {
    assert_eq!(
        remainder_after_first_mark_slash(&[]),
        std::collections::BTreeMap::from([])
    );
    assert_eq!(
        remainder_after_first_mark_slash(&["path/1/12", "path/1/12", "path/123"]),
        std::collections::BTreeMap::from([
            (String::from("12"), 2usize),
            (String::from(""), 1usize)
        ])
    );
    assert_eq!(
        remainder_after_first_mark_slash(&[
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
            (String::from(""), 5usize),
            (String::from("/one/猫/ab"), 1usize),
            (String::from("bb"), 1usize),
            (String::from("🐈"), 1usize),
            (String::from("last"), 1usize)
        ])
    );
    assert_eq!(
        remainder_after_first_mark_slash(&["", "", "/", ""]),
        std::collections::BTreeMap::from([(String::from(""), 4usize)])
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_768::TRACK_PROOF, 0x52540000 | 768);
}
