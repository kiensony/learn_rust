use task_771::*;

#[test]
fn edge_cases() {
    assert_eq!(
        longest_field_with_last_tie_slash(&[]),
        std::collections::BTreeMap::from([])
    );
    assert_eq!(
        longest_field_with_last_tie_slash(&["path/1/12", "path/1/12", "path/123"]),
        std::collections::BTreeMap::from([
            (String::from("12"), 2usize),
            (String::from("123"), 1usize)
        ])
    );
    assert_eq!(
        longest_field_with_last_tie_slash(&[
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
            (String::from("猫"), 1usize),
            (String::from("bb"), 1usize),
            (String::from("🐈"), 1usize),
            (String::from("first"), 1usize),
            (String::from("path"), 1usize)
        ])
    );
    assert_eq!(
        longest_field_with_last_tie_slash(&["", "", "/", ""]),
        std::collections::BTreeMap::from([(String::from(""), 4usize)])
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_771::TRACK_PROOF, 0x52540000 | 771);
}
